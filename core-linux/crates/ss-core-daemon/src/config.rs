//! Daemon configuration: TOML on disk + env overrides.
//!
//! The default shape ships as `core-linux/config/core.toml.default` and is
//! deployed to `/etc/screensteward/core.toml` by the packaging (Task 21).
//! In dev, the binary falls back to hard-coded defaults when the file is
//! missing.

use serde::Deserialize;
use std::path::PathBuf;

/// Root configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub ipc: Ipc,
    pub store: StoreCfg,
    pub enforce: Enforce,
    pub notifications: Notifications,
    pub logging: Logging,
}

/// Unix-socket path for the JSON-RPC server.
#[derive(Debug, Clone, Deserialize)]
pub struct Ipc {
    pub socket_path: PathBuf,
}

/// Persistent store + keyring parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct StoreCfg {
    pub db_path: PathBuf,
    pub key_fallback: PathBuf,
    pub creds_sealed: PathBuf,
    pub creds_name: String,
}

/// Enforcement / tracker knobs.
#[derive(Debug, Clone, Deserialize)]
pub struct Enforce {
    pub tick_seconds: u32,
    pub cpu_delta_threshold: u64,
    pub cgroup_root: PathBuf,
    pub gc_prev_missing_ticks: u64,
}

/// Desktop notification toggle.
#[derive(Debug, Clone, Deserialize)]
pub struct Notifications {
    pub use_desktop_notifier: bool,
}

/// Tracing verbosity (used when `RUST_LOG` is not set).
#[derive(Debug, Clone, Deserialize)]
pub struct Logging {
    pub level: String,
}

impl Config {
    /// Loads configuration from `SS_CONFIG_PATH` (if set) or
    /// `/etc/screensteward/core.toml`, then applies `SS_*` env overrides.
    ///
    /// # Errors
    ///
    /// Returns an [`anyhow::Error`] if the file cannot be read or parsed.
    pub fn load() -> anyhow::Result<Self> {
        let path = std::env::var("SS_CONFIG_PATH")
            .unwrap_or_else(|_| "/etc/screensteward/core.toml".into());
        let raw = std::fs::read_to_string(&path)?;
        let mut cfg: Self = toml::from_str(&raw)?;
        cfg.apply_env();
        Ok(cfg)
    }

    /// Applies `SS_SOCKET_PATH`, `SS_DB_PATH`, `SS_LOG_LEVEL` overrides in
    /// place. Unset variables leave the existing value untouched.
    pub fn apply_env(&mut self) {
        if let Ok(p) = std::env::var("SS_SOCKET_PATH") {
            self.ipc.socket_path = p.into();
        }
        if let Ok(p) = std::env::var("SS_DB_PATH") {
            self.store.db_path = p.into();
        }
        if let Ok(level) = std::env::var("SS_LOG_LEVEL") {
            self.logging.level = level;
        }
    }
}
