//! Child-facing IPC method handlers (no auth required).
//!
//! - `child.getStatus`      — current screen-time snapshot.
//! - `child.getDailyReport` — per-app usage for a given date.
//! - `policy.listActive`    — active policies for the child.
//! - `extension.request`    — child requests an extra-time ticket.
//! - `subscribe`            — register topics for server-push notifications.
//! - `unsubscribe`          — Phase 1 no-op.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use ss_core_model::evaluator::Action;
use ss_core_model::policy::{Policy, Rule};
use ss_core_store::dao::{child as child_dao, exception, family, policy, signature as sig_dao, usage};

use crate::app_state::AppState;
use crate::dto::{AppUsage, ChildStatus, DailyReport};
use crate::errors::{IpcError, Result};
use crate::server::{MethodContext, MethodHandler};

// ── GetStatus ──────────────────────────────────────────────────────────────

/// Returns a real-time snapshot of the child's screen-time state.
#[derive(Debug)]
pub struct GetStatus(pub AppState);

#[async_trait]
impl MethodHandler for GetStatus {
    fn name(&self) -> &'static str {
        "child.getStatus"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, _ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        let now = Utc::now();

        let Some(fam) = family::get_single(&self.0.store)? else {
            return Ok(json!({ "onboarding_required": true }));
        };

        let children = child_dao::list_by_family(&self.0.store, fam.id)?;
        let Some(first_child) = children.first() else {
            return Ok(json!({ "onboarding_required": true }));
        };

        let today = now.date_naive();
        let used = usage::minutes_for_day(&self.0.store, first_child.id, today)?;
        let policies = policy::list_active(&self.0.store, first_child.id, &now)?;

        let budget = policies
            .iter()
            .flat_map(|p| p.rules.iter())
            .find_map(|r| match r {
                Rule::DailyBudget(b) => Some(b.minutes),
                _ => None,
            });

        let (window_open, window_ends) = window_state(&policies, &now);
        let blocklist_display = collect_blocklist_display(&policies);

        let status = ChildStatus {
            today_minutes_used: used,
            today_budget_minutes: budget,
            current_window_open: window_open,
            current_window_ends_at: window_ends,
            active_blocklist_display: blocklist_display,
            session_running: !matches!(
                last_enforced_action(&policies, used, &now),
                Action::Block { .. }
            ),
        };

        Ok(serde_json::to_value(status)?)
    }
}

// ── GetDailyReport ─────────────────────────────────────────────────────────

/// Returns the usage report for a given date (defaults to today).
#[derive(Debug)]
pub struct GetDailyReport(pub AppState);

#[derive(Deserialize)]
struct DailyReportParams {
    #[serde(default)]
    date: Option<NaiveDate>,
}

#[async_trait]
impl MethodHandler for GetDailyReport {
    fn name(&self) -> &'static str {
        "child.getDailyReport"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: DailyReportParams =
            serde_json::from_value(params).unwrap_or(DailyReportParams { date: None });
        let date = p.date.unwrap_or_else(|| Utc::now().date_naive());

        let Some(fam) = family::get_single(&self.0.store)? else {
            return Ok(json!({ "onboarding_required": true }));
        };

        let children = child_dao::list_by_family(&self.0.store, fam.id)?;
        let Some(child) = children.first() else {
            return Ok(json!({ "onboarding_required": true }));
        };

        let total = usage::minutes_for_day(&self.0.store, child.id, date)?;
        // Phase 1 simplified aggregation — no pre-aggregated cache; acceptable
        // for solo dogfood scale (§7.2 approx).
        let apps = aggregate_events_by_app(&self.0.store, child.id, date)?;

        Ok(serde_json::to_value(DailyReport {
            date,
            usage_by_app: apps,
            total_minutes: total,
        })?)
    }
}

// ── ListActivePolicies ─────────────────────────────────────────────────────

/// Returns the list of currently active policies for the child.
#[derive(Debug)]
pub struct ListActivePolicies(pub AppState);

#[async_trait]
impl MethodHandler for ListActivePolicies {
    fn name(&self) -> &'static str {
        "policy.listActive"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, _ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        let Some(fam) = family::get_single(&self.0.store)? else {
            return Ok(json!({ "policies": [] }));
        };

        let children = child_dao::list_by_family(&self.0.store, fam.id)?;
        let Some(child) = children.first() else {
            return Ok(json!({ "policies": [] }));
        };

        let list = policy::list_active(&self.0.store, child.id, &Utc::now())?;
        Ok(json!({ "policies": list }))
    }
}

// ── RequestExtension ───────────────────────────────────────────────────────

/// Creates a pending extension ticket and notifies the parent.
#[derive(Debug)]
pub struct RequestExtension(pub AppState);

#[derive(Deserialize)]
struct RequestExtensionParams {
    #[serde(default)]
    reason: Option<String>,
}

#[async_trait]
impl MethodHandler for RequestExtension {
    fn name(&self) -> &'static str {
        "extension.request"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: RequestExtensionParams =
            serde_json::from_value(params).unwrap_or(RequestExtensionParams { reason: None });

        let Some(fam) = family::get_single(&self.0.store)? else {
            return Err(IpcError::Rpc("no family".into()));
        };

        let Some(child) = child_dao::list_by_family(&self.0.store, fam.id)?
            .into_iter()
            .next()
        else {
            return Err(IpcError::Rpc("no child".into()));
        };

        let id = Uuid::new_v4();
        let now = Utc::now();

        exception::insert(
            &self.0.store,
            &exception::PolicyException {
                id,
                child_id: child.id,
                granted_by_parent_id: None,
                status: exception::ExceptionStatus::Pending,
                reason: p.reason.clone(),
                duration_minutes: None,
                granted_at: None,
                expires_at: None,
                created_at: now,
            },
        )?;

        self.0.emitter.emit(
            "extensionRequested",
            json!({
                "ticket": {
                    "id": id,
                    "child_id": child.id,
                    "reason": p.reason,
                    "created_at": now,
                }
            }),
        );

        Ok(json!({ "ticket_id": id }))
    }
}

// ── Subscribe ──────────────────────────────────────────────────────────────

/// Registers topics for server-push notifications on the current connection.
#[derive(Debug)]
pub struct Subscribe;

#[derive(Deserialize)]
struct SubscribeParams {
    topics: Vec<String>,
}

#[async_trait]
impl MethodHandler for Subscribe {
    fn name(&self) -> &'static str {
        "subscribe"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: SubscribeParams = serde_json::from_value(params)
            .map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        ctx.conn_state.subscribe(p.topics.clone());
        Ok(json!({ "subscribed": p.topics }))
    }
}

// ── Unsubscribe ────────────────────────────────────────────────────────────

/// Phase 1 no-op: subscriptions are cleared on connection close.
#[derive(Debug)]
pub struct Unsubscribe;

#[async_trait]
impl MethodHandler for Unsubscribe {
    fn name(&self) -> &'static str {
        "unsubscribe"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, _ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        // Phase 1: no fine-grained removal; subscriptions live until the
        // connection closes. Implement per-topic removal in Phase 2 if needed.
        Ok(json!({ "ok": true }))
    }
}

// ── Internal helpers ───────────────────────────────────────────────────────

/// Returns `(is_open, window_ends_at)` for the first `TimeWindow` rule found
/// across all policies. If no `TimeWindow` rule exists the window is
/// considered always open.
fn window_state(
    policies: &[Policy],
    now: &DateTime<Utc>,
) -> (bool, Option<DateTime<Utc>>) {
    for p in policies {
        for r in &p.rules {
            if let Rule::TimeWindow(tw) = r {
                let open = tw.is_open(now);
                let ends = if open {
                    let date = now.date_naive();
                    Some(date.and_time(tw.end).and_utc())
                } else {
                    None
                };
                return (open, ends);
            }
        }
    }
    (true, None)
}

/// Collects human-readable labels from all `AppBlocklist` rules.
fn collect_blocklist_display(policies: &[Policy]) -> Vec<String> {
    let mut out = Vec::new();
    for p in policies {
        for r in &p.rules {
            if let Rule::AppBlocklist { matchers } = r {
                for m in matchers {
                    if let Some(b) = &m.basename {
                        out.push(b.clone());
                    } else if let Some(pg) = &m.path_glob {
                        out.push(pg.clone());
                    } else if let Some(h) = &m.content_hash {
                        out.push(format!("(hash:{})", &h[..16.min(h.len())]));
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Phase 1 stub: always returns `Allow`.
///
/// Used only to compute `session_running` on [`ChildStatus`]. A real
/// per-process evaluation requires a live candidate; this simplified version
/// considers the session blocked only when budget/window rules would block
/// any process — not implemented here.
fn last_enforced_action(
    _policies: &[Policy],
    _used: u32,
    _now: &DateTime<Utc>,
) -> Action {
    Action::Allow
}

/// Groups usage events by `(content_hash, basename)` for `date`.
///
/// Phase 1 approximation (§7.2): `minutes` is set to 0 because computing
/// accurate durations from ISO-8601 text arithmetic in `SQLite` is non-trivial.
/// The `total_minutes` field on [`DailyReport`] comes from the pre-aggregated
/// `usage_counter` table and is accurate.
///
/// # Errors
///
/// Returns [`IpcError::Store`] on `SQLite` I/O error, or [`IpcError::Store`]
/// propagated from the signature lookup.
fn aggregate_events_by_app(
    store: &ss_core_store::Store,
    child_id: Uuid,
    date: NaiveDate,
) -> Result<Vec<AppUsage>> {
    let start = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
    let end = date.and_hms_opt(23, 59, 59).unwrap().and_utc();

    let rows = store.with_conn(|c| {
        let mut stmt = c.prepare(
            "SELECT content_hash, basename
             FROM usage_event
             WHERE child_id = ?1 AND started_at >= ?2 AND started_at <= ?3
             GROUP BY content_hash, basename",
        )?;
        let res = stmt.query_map(
            rusqlite::params![child_id.to_string(), start.to_rfc3339(), end.to_rfc3339()],
            |r| {
                let h: String = r.get(0)?;
                let b: String = r.get(1)?;
                Ok((h, b))
            },
        )?;
        let rows: Vec<(String, String)> = res.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok::<_, ss_core_store::StoreError>(rows)
    })?;

    let mut out = Vec::new();
    for (hash, basename) in rows {
        let display_name = sig_dao::get(store, &hash)?.and_then(|s| s.display_name);
        out.push(AppUsage {
            content_hash: hash,
            display_name,
            basename,
            minutes: 0, // approx §7.2
        });
    }
    Ok(out)
}
