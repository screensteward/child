//! Orchestrator that dispatches policy [`Action`]s onto a [`CgroupBackend`].
//!
//! For each observed process, the evaluator produces an [`Action`]. The
//! enforcer turns those abstract decisions into concrete cgroup operations
//! (ensure scope, move PID, freeze / unfreeze / kill).
//!
//! On [`Action::Block`] the primary mechanism is `freeze`; if that fails we
//! fall back to `kill_scope` so a misbehaving or locked cgroup cannot leave
//! a blocked app running.

use std::sync::Arc;
use tracing::{error, info, warn};

use crate::cgroup::{AppScopeId, CgroupBackend};
use crate::errors::Result;
use crate::proc_scan::ObservedProcess;
use ss_core_model::evaluator::Action;

/// A single enforcement request: the process observed this tick and the
/// [`Action`] the evaluator wants applied to it.
#[derive(Debug)]
pub struct EnforcerContext<'a> {
    pub process: &'a ObservedProcess,
    pub action: Action,
}

/// Dispatches [`Action`]s onto an injected [`CgroupBackend`].
///
/// Cheap to clone via the inner `Arc`.
pub struct Enforcer {
    cgroup: Arc<dyn CgroupBackend>,
}

impl std::fmt::Debug for Enforcer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enforcer").finish_non_exhaustive()
    }
}

impl Enforcer {
    /// Creates a new enforcer backed by the given cgroup implementation.
    #[must_use]
    pub fn new(cgroup: Arc<dyn CgroupBackend>) -> Self {
        Self { cgroup }
    }

    /// Applies the action in `ctx` to the backing cgroup backend.
    ///
    /// The signature is `async` even though [`CgroupBackend`] is currently
    /// synchronous: this keeps the door open for swapping to an async backend
    /// (see `CgroupBackendAsync`) without a caller-side refactor.
    ///
    /// # Errors
    ///
    /// Returns an error if a required cgroup operation fails. `freeze`
    /// failures on [`Action::Block`] are logged and recovered from via
    /// `kill_scope`; only unrecoverable backend errors propagate.
    // `async` is intentional: we want to keep the door open for an async
    // `CgroupBackend` without a caller-side refactor (see `CgroupBackendAsync`).
    #[allow(clippy::unused_async)]
    pub async fn apply(&self, ctx: &EnforcerContext<'_>) -> Result<()> {
        let id = scope_id(&ctx.process.content_hash);
        match &ctx.action {
            Action::Allow => {
                // Only thaw if a scope is actually frozen (recovery from a
                // previous Block when the user earns time back / new day /
                // window reopens).
                if self.cgroup.scope_exists(&id) && self.cgroup.is_frozen(&id) {
                    info!(pid = ctx.process.pid, scope = %id, "unfreezing after Allow");
                    self.cgroup.unfreeze(&id)?;
                }
            }
            Action::Warn {
                reason,
                remaining_minutes,
            } => {
                // Notifier is decoupled (dispatched by the caller). Here we
                // just make sure the scope exists and the PID is in it, so a
                // subsequent Block tick freezes instantly.
                info!(
                    pid = ctx.process.pid,
                    reason = %reason,
                    remaining = %remaining_minutes,
                    "warn",
                );
                self.ensure_and_move(&id, ctx.process.pid)?;
            }
            Action::Block { reason } => {
                info!(pid = ctx.process.pid, reason = %reason, scope = %id, "block -> freeze");
                self.ensure_and_move(&id, ctx.process.pid)?;
                if let Err(e) = self.cgroup.freeze(&id) {
                    warn!(error = %e, "freeze failed - falling back to kill_scope");
                    if let Err(e2) = self.cgroup.kill_scope(&id) {
                        error!(error = %e2, "kill_scope also failed");
                    }
                }
            }
        }
        Ok(())
    }

    fn ensure_and_move(&self, id: &AppScopeId, pid: i32) -> Result<()> {
        self.cgroup.ensure_scope(id)?;
        // Best effort: a process may have died between the /proc scan and now.
        let _ = self.cgroup.move_pid(id, pid);
        Ok(())
    }
}

/// Derives an [`AppScopeId`] from a `content_hash` of the form `sha256:<hex>`.
///
/// The resulting id is `app-` followed by the first 16 hex characters of the
/// digest (or fewer if the hash is shorter — mostly useful in tests).
#[must_use]
pub fn scope_id(content_hash: &str) -> AppScopeId {
    let hex = content_hash.trim_start_matches("sha256:");
    let short = &hex[..16.min(hex.len())];
    AppScopeId(format!("app-{short}"))
}
