//! Config parsing tests.
//!
//! Checked against the actual `core.toml.default` file to catch schema drift:
//! any breaking change to the struct definitions has to be mirrored in the
//! shipped default, otherwise these tests fail.

use ss_core_daemon::config::Config;

#[test]
fn default_config_parses() {
    let toml = include_str!("../../../config/core.toml.default");
    let c: Config = toml::from_str(toml).expect("default TOML must parse");
    assert_eq!(
        c.ipc.socket_path,
        std::path::PathBuf::from("/run/screensteward.sock")
    );
    assert_eq!(c.enforce.tick_seconds, 5);
}

#[test]
fn env_overrides_socket_path() {
    // Isolate from other tests; SetEnv touches process-wide state.
    std::env::set_var("SS_SOCKET_PATH", "/tmp/ss.sock");
    let toml = include_str!("../../../config/core.toml.default");
    let mut c: Config = toml::from_str(toml).expect("default TOML must parse");
    c.apply_env();
    assert_eq!(c.ipc.socket_path, std::path::PathBuf::from("/tmp/ss.sock"));
    std::env::remove_var("SS_SOCKET_PATH");
}
