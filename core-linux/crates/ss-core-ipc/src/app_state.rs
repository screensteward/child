//! Global state shared by handlers: Store, notification emitter,
//! uptime tracker, immutable config.

use std::sync::Arc;
use std::time::Instant;

use ss_core_store::Store;

use crate::server::NotificationEmitter;

/// Application-wide state cloned into each handler.
///
/// `AppState` is `Clone` because handlers hold it by value (each handler is
/// constructed with `state.clone()` in the dispatch table). The inner `Store`
/// and `NotificationEmitter` both wrap `Arc`, so cloning is cheap.
#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub emitter: NotificationEmitter,
    pub started_at: Arc<Instant>,
    pub tpm_used: bool,
    pub version: String,
}

impl AppState {
    /// Construct a fresh [`AppState`].
    ///
    /// `started_at` is captured as the current [`Instant`] at construction.
    #[must_use]
    pub fn new(
        store: Store,
        emitter: NotificationEmitter,
        tpm_used: bool,
        version: impl Into<String>,
    ) -> Self {
        Self {
            store,
            emitter,
            started_at: Arc::new(Instant::now()),
            tpm_used,
            version: version.into(),
        }
    }

    /// Seconds elapsed since the daemon started.
    #[must_use]
    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("tpm_used", &self.tpm_used)
            .field("version", &self.version)
            .finish_non_exhaustive()
    }
}
