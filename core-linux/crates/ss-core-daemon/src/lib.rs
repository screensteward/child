//! `ScreenSteward` Core daemon library surface.
//!
//! Exposes the modules needed both by the `screensteward-core` binary and by
//! integration tests: [`config`] (TOML + env overrides), [`signals`]
//! (SIGTERM/SIGINT → [`tokio_util::sync::CancellationToken`]), and
//! [`supervisor`] (the tokio tick loop tying `/proc` scan, G-Counter,
//! evaluator and enforcer together).

pub mod config;
pub mod signals;
pub mod supervisor;
