//! Graceful-shutdown plumbing.
//!
//! [`wait_for_shutdown`] spawns a detached task that waits on SIGTERM and
//! SIGINT and triggers the returned [`CancellationToken`] when either fires.
//! Callers plug the token into `tokio::select!` alongside the server loop and
//! the supervisor tick loop.

use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;

/// Returns a [`CancellationToken`] that fires on SIGTERM or SIGINT.
///
/// Must be called from within a tokio runtime (uses [`tokio::spawn`] for
/// the watcher task and installs the signal handlers via
/// [`signal`]).
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if either signal handler cannot be
/// installed (typically: running in an environment where signals are
/// unsupported, e.g. certain embedded runtimes).
pub fn wait_for_shutdown() -> anyhow::Result<CancellationToken> {
    let token = CancellationToken::new();
    let child = token.clone();
    let mut term = signal(SignalKind::terminate())?;
    let mut intr = signal(SignalKind::interrupt())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = term.recv() => tracing::info!("received SIGTERM"),
            _ = intr.recv() => tracing::info!("received SIGINT"),
        }
        child.cancel();
    });
    Ok(token)
}
