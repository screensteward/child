use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::{IpcError, Result};
use crate::server::{MethodContext, MethodHandler};
use ss_core_model::policy::{Policy, Rule, Scope};
use ss_core_store::dao;

// ── policy.create ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Create(pub AppState);

#[derive(Deserialize)]
struct CreateParams {
    child_id: Uuid,
    scope: Scope,
    rules: Vec<Rule>,
    #[serde(default)]
    priority: Option<i32>,
    #[serde(default)]
    active_from: Option<chrono::DateTime<Utc>>,
    #[serde(default)]
    active_until: Option<chrono::DateTime<Utc>>,
}

#[async_trait]
impl MethodHandler for Create {
    fn name(&self) -> &'static str {
        "policy.create"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: CreateParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let now = Utc::now();
        let pol = Policy {
            id: Uuid::new_v4(),
            child_id: p.child_id,
            scope: p.scope,
            rules: p.rules,
            priority: p.priority.unwrap_or(0),
            active_from: p.active_from,
            active_until: p.active_until,
            created_at: now,
            modified_at: now,
        };
        dao::policy::insert(&self.0.store, &pol)?;
        ctx.notifier.emit("policyChanged", json!({ "policy_id": pol.id }));
        Ok(json!({ "id": pol.id }))
    }
}

// ── policy.update ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Update(pub AppState);

#[derive(Deserialize)]
struct UpdateParams {
    id: Uuid,
    #[serde(default)]
    scope: Option<Scope>,
    #[serde(default)]
    rules: Option<Vec<Rule>>,
    #[serde(default)]
    priority: Option<i32>,
    #[serde(default)]
    active_from: Option<chrono::DateTime<Utc>>,
    #[serde(default)]
    active_until: Option<chrono::DateTime<Utc>>,
}

#[async_trait]
impl MethodHandler for Update {
    fn name(&self) -> &'static str {
        "policy.update"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    #[allow(clippy::too_many_lines)]
    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: UpdateParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        // Load the existing policy via raw SQL (dao::policy has no get_by_id).
        let existing: Option<Policy> = self.0.store.with_conn(|c| {
            let mut st = c.prepare(
                "SELECT id,child_id,scope_json,rules_json,priority,active_from,active_until,created_at,modified_at
                 FROM policy WHERE id = ?1",
            )?;
            let r = st.query_row(rusqlite::params![p.id.to_string()], |row| {
                let scope: Scope =
                    serde_json::from_str(&row.get::<_, String>(2)?).expect("invariant: scope JSON");
                let rules: Vec<Rule> =
                    serde_json::from_str(&row.get::<_, String>(3)?).expect("invariant: rules JSON");
                Ok(Policy {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).expect("invariant: uuid"),
                    child_id: Uuid::parse_str(&row.get::<_, String>(1)?).expect("invariant: uuid"),
                    scope,
                    rules,
                    priority: row.get(4)?,
                    active_from: row
                        .get::<_, Option<String>>(5)?
                        .map(|s| chrono::DateTime::parse_from_rfc3339(&s).expect("invariant: ts").with_timezone(&Utc)),
                    active_until: row
                        .get::<_, Option<String>>(6)?
                        .map(|s| chrono::DateTime::parse_from_rfc3339(&s).expect("invariant: ts").with_timezone(&Utc)),
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                        .expect("invariant: ts")
                        .with_timezone(&Utc),
                    modified_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .expect("invariant: ts")
                        .with_timezone(&Utc),
                })
            });
            match r {
                Ok(pol) => Ok(Some(pol)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })?;
        let mut existing = existing.ok_or_else(|| IpcError::Rpc("policy not found".into()))?;
        if let Some(s) = p.scope {
            existing.scope = s;
        }
        if let Some(r) = p.rules {
            existing.rules = r;
        }
        if let Some(pr) = p.priority {
            existing.priority = pr;
        }
        if p.active_from.is_some() {
            existing.active_from = p.active_from;
        }
        if p.active_until.is_some() {
            existing.active_until = p.active_until;
        }
        existing.modified_at = Utc::now();
        dao::policy::update(&self.0.store, &existing)?;
        ctx.notifier
            .emit("policyChanged", json!({ "policy_id": existing.id }));
        Ok(json!({ "ok": true }))
    }
}

// ── policy.delete ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Delete(pub AppState);

#[derive(Deserialize)]
struct DeleteParams {
    id: Uuid,
}

#[async_trait]
impl MethodHandler for Delete {
    fn name(&self) -> &'static str {
        "policy.delete"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: DeleteParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        dao::policy::delete(&self.0.store, p.id)?;
        ctx.notifier
            .emit("policyChanged", json!({ "policy_id": p.id, "deleted": true }));
        Ok(json!({ "ok": true }))
    }
}

// ── policy.list ─────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct List(pub AppState);

#[derive(Deserialize)]
struct ListParams {
    #[serde(default)]
    child_id: Option<Uuid>,
}

#[async_trait]
impl MethodHandler for List {
    fn name(&self) -> &'static str {
        "policy.list"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: ListParams =
            serde_json::from_value(params).unwrap_or(ListParams { child_id: None });
        let child_id = if let Some(id) = p.child_id {
            id
        } else {
            let fam = dao::family::get_single(&self.0.store)?
                .ok_or_else(|| IpcError::Rpc("no family".into()))?;
            let children = dao::child::list_by_family(&self.0.store, fam.id)?;
            children
                .first()
                .map(|c| c.id)
                .ok_or_else(|| IpcError::Rpc("no child".into()))?
        };
        let list = dao::policy::list_by_child(&self.0.store, child_id)?;
        Ok(json!({ "policies": list }))
    }
}
