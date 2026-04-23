//! Idle detection sources for the enforcement loop.
//!
//! Two real sources are provided:
//! - [`LogindIdleSource`][] reads `IdleHint` on the active graphical
//!   session via `org.freedesktop.login1` on the **system** bus. This is
//!   the authoritative source in production (the daemon runs as a system
//!   service).
//! - [`ScreenSaverIdleSource`][] calls
//!   `org.freedesktop.ScreenSaver.GetActive` on the **session** bus. Only
//!   available when the daemon has access to a session bus (typically dev
//!   mode running as a user service); acts as a fallback.
//!
//! Sources are aggregated by [`IdleDetector`] with OR semantics: the user
//! is considered idle as soon as at least one source reports idle. Errors
//! from a source are logged via `warn!` but do not abort the check.

use async_trait::async_trait;
use tracing::warn;
use zbus::Connection;

use crate::errors::{EnforceError, Result};

#[async_trait]
pub trait IdleSource: Send + Sync + std::fmt::Debug {
    async fn is_idle(&self) -> Result<bool>;
    fn name(&self) -> &'static str;
}

/// Aggregates multiple idle sources. Global idle = at least one source
/// reports idle. Errors are logged and do not contribute.
#[derive(Debug)]
pub struct IdleDetector {
    sources: Vec<Box<dyn IdleSource>>,
}

impl IdleDetector {
    #[must_use]
    pub fn new(sources: Vec<Box<dyn IdleSource>>) -> Self {
        Self { sources }
    }

    /// Returns `Ok(true)` as soon as any source reports idle. Source
    /// errors are logged and skipped.
    ///
    /// # Errors
    ///
    /// Currently infallible — source errors are swallowed and logged —
    /// but the `Result` return type is preserved for forward
    /// compatibility.
    pub async fn is_idle_now(&self) -> Result<bool> {
        for s in &self.sources {
            match s.is_idle().await {
                Ok(true) => return Ok(true),
                Ok(false) => {}
                Err(e) => warn!(source = %s.name(), error = %e, "idle source failed"),
            }
        }
        Ok(false)
    }
}

/// Reads `IdleHint` on the currently active session via logind.
#[derive(Debug)]
pub struct LogindIdleSource {
    conn: Connection,
}

impl LogindIdleSource {
    /// Connects to the system bus.
    ///
    /// # Errors
    ///
    /// Returns [`EnforceError::DBus`] if the system bus is not
    /// available.
    pub async fn connect() -> Result<Self> {
        let conn = Connection::system()
            .await
            .map_err(|e| EnforceError::DBus(e.to_string()))?;
        Ok(Self { conn })
    }
}

#[async_trait]
impl IdleSource for LogindIdleSource {
    fn name(&self) -> &'static str {
        "logind"
    }

    async fn is_idle(&self) -> Result<bool> {
        // Enumerate sessions via Manager.ListSessions() and read
        // IdleHint on the first Active==true session (graphical seat0
        // session in the typical case).
        let manager = zbus::Proxy::new(
            &self.conn,
            "org.freedesktop.login1",
            "/org/freedesktop/login1",
            "org.freedesktop.login1.Manager",
        )
        .await
        .map_err(|e| EnforceError::DBus(e.to_string()))?;

        let sessions: Vec<(String, u32, String, String, zbus::zvariant::OwnedObjectPath)> =
            manager
                .call("ListSessions", &())
                .await
                .map_err(|e| EnforceError::DBus(e.to_string()))?;

        for (_id, _uid, _user, _seat, path) in sessions {
            let session = zbus::Proxy::new(
                &self.conn,
                "org.freedesktop.login1",
                path.as_str(),
                "org.freedesktop.login1.Session",
            )
            .await
            .map_err(|e| EnforceError::DBus(e.to_string()))?;

            let is_active: bool = session
                .get_property("Active")
                .await
                .map_err(|e| EnforceError::DBus(e.to_string()))?;
            if !is_active {
                continue;
            }

            let idle_hint: bool = session
                .get_property("IdleHint")
                .await
                .map_err(|e| EnforceError::DBus(e.to_string()))?;
            return Ok(idle_hint);
        }
        Ok(false)
    }
}

/// Fallback: `org.freedesktop.ScreenSaver.GetActive` on the session bus.
///
/// A system daemon does not have access to a session bus by default, so
/// this source is mostly useful when running the daemon as a user-mode
/// service in dev. Production relies on [`LogindIdleSource`].
#[derive(Debug)]
pub struct ScreenSaverIdleSource {
    conn: Connection,
}

impl ScreenSaverIdleSource {
    /// Connects to the session bus.
    ///
    /// # Errors
    ///
    /// Returns [`EnforceError::DBus`] if no session bus is reachable
    /// (typical when running as a system service).
    pub async fn connect_session() -> Result<Self> {
        let conn = Connection::session()
            .await
            .map_err(|e| EnforceError::DBus(e.to_string()))?;
        Ok(Self { conn })
    }
}

#[async_trait]
impl IdleSource for ScreenSaverIdleSource {
    fn name(&self) -> &'static str {
        "screensaver"
    }

    async fn is_idle(&self) -> Result<bool> {
        let p = zbus::Proxy::new(
            &self.conn,
            "org.freedesktop.ScreenSaver",
            "/org/freedesktop/ScreenSaver",
            "org.freedesktop.ScreenSaver",
        )
        .await
        .map_err(|e| EnforceError::DBus(e.to_string()))?;
        let active: bool = p
            .call("GetActive", &())
            .await
            .map_err(|e| EnforceError::DBus(e.to_string()))?;
        Ok(active)
    }
}
