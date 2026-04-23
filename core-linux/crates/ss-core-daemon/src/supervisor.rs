//! Enforcement supervisor: one tick every `enforce.tick_seconds`.
//!
//! Each tick:
//! 1. reads an idle hint,
//! 2. scans `/proc` into [`ObservedProcess`]es,
//! 3. advances the [`Tracker`] and persists `(content_hash, basename, path)`
//!    observations,
//! 4. raises `unknown_binary` warnings when a known basename appears with a
//!    never-before-seen `content_hash`,
//! 5. folds the tick's credited seconds into the G-Counter
//!    (`usage_counter.minutes_used`) and emits `usageUpdate` when the
//!    absolute minute count changes,
//! 6. evaluates each process against the active policies, dispatches the
//!    resulting [`Action`] to the [`Enforcer`], notifies on `Warn`, emits
//!    `enforcementAction` on `Block`.
//!
//! Shutdown is driven by a [`CancellationToken`] passed to [`Supervisor::run`].
//! The token is checked every tick via `tokio::select!`; the loop exits
//! without waiting for the next `interval.tick()`.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::time::{interval, MissedTickBehavior};
use tokio_util::sync::CancellationToken;

use ss_core_enforce::{
    cgroup::CgroupV2,
    enforcer::{Enforcer, EnforcerContext},
    idle::{IdleDetector, IdleSource, LogindIdleSource},
    notifier::{DBusNotifier, DesktopNotifier, LogNotifier},
    proc_scan::{scan_processes, ContentHashCache, ObservedProcess},
    tracker::{PrevCpu, Tracker},
};
use ss_core_ipc::server::NotificationEmitter;
use ss_core_model::evaluator::{evaluate, Action, ProcessCandidate};
use ss_core_store::dao;
use ss_core_store::Store;

use crate::config::Config;

/// Long-lived enforcement loop. Owns the store handle, the emitter (so it
/// can push `usageUpdate` / `enforcementAction`) and the resolved config.
pub struct Supervisor {
    pub store: Store,
    pub emitter: NotificationEmitter,
    pub config: Config,
}

impl std::fmt::Debug for Supervisor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Supervisor")
            .field("tick_seconds", &self.config.enforce.tick_seconds)
            .finish_non_exhaustive()
    }
}

impl Supervisor {
    /// Drives the tick loop until `token` fires.
    ///
    /// # Errors
    ///
    /// Returns an [`anyhow::Error`] if a fatal (non-recoverable) error
    /// surfaces. Individual tick failures are logged at `warn!` and the
    /// loop keeps going — transient `/proc` or D-Bus hiccups must not
    /// tear down the daemon.
    pub async fn run(self, token: CancellationToken) -> anyhow::Result<()> {
        let tick_seconds = self.config.enforce.tick_seconds;
        let cpu_delta_threshold = self.config.enforce.cpu_delta_threshold;
        let cgroup_root = self.config.enforce.cgroup_root.clone();
        let use_desktop_notifier = self.config.notifications.use_desktop_notifier;

        let cache = ContentHashCache::new();
        let mut prev: HashMap<i32, PrevCpu> = HashMap::new();

        let cgroup = Arc::new(CgroupV2::new(cgroup_root));
        let enforcer = Enforcer::new(cgroup.clone());

        let notifier: Arc<dyn DesktopNotifier> = if use_desktop_notifier {
            Arc::new(DBusNotifier::connect_session().await)
        } else {
            Arc::new(LogNotifier)
        };

        let idle = {
            let mut sources: Vec<Box<dyn IdleSource>> = Vec::new();
            match LogindIdleSource::connect().await {
                Ok(s) => sources.push(Box::new(s)),
                Err(e) => tracing::warn!(error = %e, "no logind idle source"),
            }
            IdleDetector::new(sources)
        };

        let mut tracker = Tracker::new(tick_seconds, cpu_delta_threshold);
        let mut ticker = interval(Duration::from_secs(u64::from(tick_seconds)));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        tracing::info!(tick_seconds, "supervisor loop started");
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if let Err(e) = self
                        .tick_once(&cache, &mut prev, &enforcer, notifier.as_ref(), &idle, &mut tracker)
                        .await
                    {
                        tracing::warn!(error = %e, "tick failed");
                    }
                }
                () = token.cancelled() => {
                    tracing::info!("supervisor shutting down");
                    break;
                }
            }
        }
        Ok(())
    }

    /// Runs one tick end-to-end. Errors are propagated to the caller (the
    /// run loop logs them and continues).
    //
    // The tick is intentionally kept as a single narrative to make the
    // sequence of steps (scan → persist → evaluate → enforce → emit)
    // obvious. Splitting would fragment the state threaded through each
    // step without removing the shared local variables.
    #[allow(clippy::too_many_lines)]
    async fn tick_once(
        &self,
        cache: &ContentHashCache,
        prev: &mut HashMap<i32, PrevCpu>,
        enforcer: &Enforcer,
        notifier: &dyn DesktopNotifier,
        idle: &IdleDetector,
        tracker: &mut Tracker,
    ) -> anyhow::Result<()> {
        let idle_now = idle.is_idle_now().await.unwrap_or(false);
        let processes: Vec<ObservedProcess> = scan_processes(cache)?;
        let outcome = tracker.tick(&processes, prev, idle_now);

        // Persist app_signature observations for every running process. The
        // DAO dedupes basenames and paths; redundant writes are cheap.
        for p in &processes {
            if let Err(e) = dao::signature::upsert_observation(
                &self.store,
                &p.content_hash,
                &p.basename,
                &p.exe_path.to_string_lossy(),
            ) {
                tracing::warn!(error = %e, content_hash = %p.content_hash, "upsert_observation failed");
            }
        }

        // unknown_binary warnings: a hash is "unknown" when its basename was
        // previously observed with a DIFFERENT hash. Lookup is done against
        // the persisted signatures, not the in-memory tracker, so the signal
        // survives daemon restarts.
        for new_hash in &outcome.newly_seen_hashes {
            if let Some(p) = processes.iter().find(|p| &p.content_hash == new_hash) {
                match dao::signature::find_by_basename(&self.store, &p.basename) {
                    Ok(rivals) => {
                        let had_other = rivals.iter().any(|s| s.content_hash != *new_hash);
                        if had_other {
                            self.emitter.emit(
                                "enforcementAction",
                                serde_json::json!({
                                    "action": "warn",
                                    "reason": "unknown_binary",
                                    "basename": p.basename,
                                    "path": p.exe_path.to_string_lossy(),
                                    "content_hash": new_hash,
                                }),
                            );
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, basename = %p.basename, "find_by_basename failed");
                    }
                }
            }
        }

        tracker.finalize_tick(&outcome);
        tracker.gc_prev(prev, self.config.enforce.gc_prev_missing_ticks);

        // Phase 1: one family, one child, one device. If any of those rows
        // are missing the daemon is still bootstrapping (parent hasn't run
        // family.bootstrap yet) and we skip the evaluator half of the tick.
        let Some(fam) = dao::family::get_single(&self.store)? else {
            return Ok(());
        };
        let Some(child) = dao::child::list_by_family(&self.store, fam.id)?
            .into_iter()
            .next()
        else {
            return Ok(());
        };
        let Some(device) = dao::device::list_by_child(&self.store, child.id)?
            .into_iter()
            .next()
        else {
            return Ok(());
        };

        let now = chrono::Utc::now();
        let policies = dao::policy::list_active(&self.store, child.id, &now)?;

        // G-Counter: fold the tick's credited seconds into the absolute
        // minute count for today. Only emit `usageUpdate` when the integer
        // minute value actually moves, to keep the broadcast quiet.
        let today = now.date_naive();
        let minutes_used_before = dao::usage::minutes_for_day(&self.store, child.id, today)?;
        let added_seconds: u32 = outcome.seconds_by_hash.iter().map(|(_, s)| s).sum();
        let new_minutes = minutes_used_before.saturating_add(added_seconds / 60);
        if new_minutes != minutes_used_before {
            dao::usage::upsert_minutes(&self.store, child.id, device.id, today, new_minutes)?;
            self.emitter.emit(
                "usageUpdate",
                serde_json::json!({
                    "child_id": child.id,
                    "total_minutes_today": new_minutes,
                }),
            );
        }

        // Evaluate + enforce per observed process.
        for p in &processes {
            let candidate = ProcessCandidate {
                content_hash: Some(p.content_hash.clone()),
                basename: p.basename.clone(),
                path: p.exe_path.to_string_lossy().to_string(),
            };
            let action = evaluate(&policies, &candidate, new_minutes, &now);
            if let Action::Warn {
                reason,
                remaining_minutes,
            } = &action
            {
                let body = format!("{reason} — {remaining_minutes} min restantes");
                if let Err(e) = notifier.notify("ScreenSteward", &body).await {
                    tracing::warn!(error = %e, "desktop notifier failed");
                }
            }
            enforcer
                .apply(&EnforcerContext {
                    process: p,
                    action: action.clone(),
                })
                .await?;
            if let Action::Block { reason } = action {
                self.emitter.emit(
                    "enforcementAction",
                    serde_json::json!({
                        "action": "freeze",
                        "reason": reason,
                        "basename": p.basename,
                        "path": p.exe_path.to_string_lossy(),
                        "content_hash": p.content_hash,
                    }),
                );
            }
        }
        Ok(())
    }
}
