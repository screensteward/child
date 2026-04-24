//! Desktop notification abstraction.
//!
//! Enforcement actions that need to inform the human user (e.g. "5 minutes
//! left on `YouTube`") raise a desktop notification via the freedesktop.org
//! spec. Because the daemon may run as a **system** service without access
//! to a session bus, the real implementation degrades gracefully to a no-op
//! when the session bus is unreachable. For headless tests or fallback use
//! cases, [`LogNotifier`] simply writes to the tracing pipeline.

use std::collections::HashMap;

use async_trait::async_trait;
use tracing::warn;

use crate::errors::{EnforceError, Result};

/// Abstraction over a desktop-notification backend.
#[async_trait]
pub trait DesktopNotifier: Send + Sync {
    /// Displays a notification with the given `summary` and `body`.
    ///
    /// # Errors
    ///
    /// Returns [`EnforceError::DBus`] if an underlying transport error
    /// occurs. Implementations are allowed to silently no-op (return
    /// `Ok(())`) when the backend is unavailable — this is not considered
    /// an error.
    async fn notify(&self, summary: &str, body: &str) -> Result<()>;
}

/// `org.freedesktop.Notifications` implementation.
///
/// Silently no-ops when the session bus is unreachable (typical when the
/// daemon runs as a system service with no logged-in session).
pub struct DBusNotifier {
    conn: Option<zbus::Connection>,
}

impl std::fmt::Debug for DBusNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DBusNotifier")
            .field("conn", &self.conn.is_some())
            .finish()
    }
}

impl DBusNotifier {
    /// Attempts to connect to the user session bus. On failure, logs a
    /// warning and returns an instance whose `notify()` is a no-op.
    pub async fn connect_session() -> Self {
        match zbus::Connection::session().await {
            Ok(c) => Self { conn: Some(c) },
            Err(e) => {
                warn!(error = %e, "no session bus for desktop notifications -- notifs disabled");
                Self { conn: None }
            }
        }
    }

    /// Returns `true` when a live session bus connection is held.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.conn.is_some()
    }
}

#[async_trait]
impl DesktopNotifier for DBusNotifier {
    async fn notify(&self, summary: &str, body: &str) -> Result<()> {
        let Some(conn) = &self.conn else {
            return Ok(());
        };
        let p = zbus::Proxy::new(
            conn,
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
        )
        .await
        .map_err(|e| EnforceError::DBus(e.to_string()))?;

        // Empty hints map: the lifetime is free since there are no borrowed
        // `Value`s inside; `'static` is the simplest pick.
        let hints: HashMap<&str, zbus::zvariant::Value<'static>> = HashMap::new();

        let _id: u32 = p
            .call(
                "Notify",
                &(
                    "ScreenSteward",      // app_name
                    0u32,                 // replaces_id
                    "dialog-information", // app_icon
                    summary,
                    body,
                    Vec::<String>::new(), // actions
                    hints,                // hints
                    5000i32,              // expire_timeout_ms
                ),
            )
            .await
            .map_err(|e| EnforceError::DBus(e.to_string()))?;
        Ok(())
    }
}

/// Fallback notifier that logs to `tracing` instead of touching D-Bus.
///
/// Useful in headless environments and as a default during Phase 1 dogfood.
#[derive(Debug, Default, Clone, Copy)]
pub struct LogNotifier;

#[async_trait]
impl DesktopNotifier for LogNotifier {
    async fn notify(&self, summary: &str, body: &str) -> Result<()> {
        tracing::info!(summary, body, "desktop-notification");
        Ok(())
    }
}
