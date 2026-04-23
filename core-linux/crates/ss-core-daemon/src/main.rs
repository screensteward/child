//! `screensteward-core` binary entry point.
//!
//! Wires together config, keyring, store, IPC server and enforcement
//! supervisor. A single [`NotificationEmitter`] is constructed up front and
//! shared between the server (which fans out to subscribed connections) and
//! the `AppState` passed to every handler — without that sharing the
//! handlers would emit into a dead channel.
//!
//! The binary blocks on `tokio::select!` over:
//! * the server accept loop,
//! * the supervisor tick loop,
//! * the SIGTERM/SIGINT [`CancellationToken`].
//!
//! Whichever branch completes first triggers a clean shutdown of the others
//! (the server's `accept` loop is cancel-safe; dropping it simply closes the
//! `UnixListener`).

use std::path::PathBuf;

use ss_core_daemon::config::{self, Config, Enforce, Ipc, Logging, Notifications, StoreCfg};
use ss_core_daemon::signals::wait_for_shutdown;
use ss_core_daemon::supervisor::Supervisor;
use ss_core_ipc::app_state::AppState;
use ss_core_ipc::methods::registry;
use ss_core_ipc::server::{NotificationEmitter, Server};
use ss_core_store::keyring::{FallbackKeyring, Keyring, LinuxKeyring, SystemdCredsKeyring};
use ss_core_store::Store;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let cfg: Config = config::Config::load().unwrap_or_else(|e| {
        tracing::warn!(error = %e, "no config file — using built-in defaults (dev mode)");
        default_dev_config()
    });

    let keyring = LinuxKeyring {
        systemd_creds: SystemdCredsKeyring {
            name: cfg.store.creds_name.clone(),
            sealed_path: cfg.store.creds_sealed.clone(),
        },
        fallback: FallbackKeyring::new(cfg.store.key_fallback.clone()),
    };
    let key = keyring.load_or_create()?;
    let tpm_used = cfg.store.creds_sealed.exists();
    let store = Store::open_with_key(&cfg.store.db_path, &key)?;

    // Shutdown token: installed before the server / supervisor so a SIGINT
    // during bootstrap cancels both branches cleanly.
    let shutdown = wait_for_shutdown()?;

    // CRITICAL: build the emitter ONCE and pass the same value to the
    // AppState (handlers) AND the Server (fan-out writer). If we used
    // `Server::new` the server would create a fresh emitter, and anything
    // a handler emits would go into the void.
    let emitter = NotificationEmitter::new();
    let state = AppState::new(
        store.clone(),
        emitter.clone(),
        tpm_used,
        env!("CARGO_PKG_VERSION"),
    );
    let handlers = registry(state);
    let server = Server::with_emitter(cfg.ipc.socket_path.clone(), handlers, emitter.clone());

    let sup = Supervisor {
        store,
        emitter,
        config: cfg,
    };

    let server_fut = server.serve();
    let sup_fut = sup.run(shutdown.clone());

    tokio::select! {
        r = server_fut => r.map_err(|e| anyhow::anyhow!("server: {e}"))?,
        r = sup_fut => r?,
        () = shutdown.cancelled() => {},
    }
    tracing::info!("bye");
    Ok(())
}

/// Installs a `tracing` subscriber honouring `RUST_LOG`. Idempotent: a
/// second call is a no-op (`set_global_default` returns an error we
/// deliberately drop).
fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into());
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

/// Fallback used when `/etc/screensteward/core.toml` is missing. Picks
/// socket/DB paths that do not require root so the binary is runnable in
/// dev (`SS_SOCKET_PATH` / `SS_DB_PATH` still override).
fn default_dev_config() -> Config {
    Config {
        ipc: Ipc {
            socket_path: PathBuf::from(
                std::env::var("SS_SOCKET_PATH")
                    .unwrap_or_else(|_| "/tmp/screensteward.sock".into()),
            ),
        },
        store: StoreCfg {
            db_path: PathBuf::from(
                std::env::var("SS_DB_PATH").unwrap_or_else(|_| "./local/core.db".into()),
            ),
            key_fallback: PathBuf::from("./local/master.key"),
            creds_sealed: PathBuf::from("./local/master.cred"),
            creds_name: "screensteward-master-key-dev".into(),
        },
        enforce: Enforce {
            tick_seconds: 5,
            cpu_delta_threshold: 5,
            cgroup_root: PathBuf::from("/sys/fs/cgroup/screensteward.slice"),
            gc_prev_missing_ticks: 12,
        },
        notifications: Notifications {
            use_desktop_notifier: false,
        },
        logging: Logging {
            level: "debug".into(),
        },
    }
}
