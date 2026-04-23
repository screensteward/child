//! JSON-RPC 2.0 IPC server for `ScreenSteward` Core.

pub mod app_state;
pub mod auth;
pub mod dto;
pub mod errors;
pub mod framing;
pub mod idempotency;
pub mod methods;
pub mod rpc;
pub mod server;

pub use app_state::AppState;
pub use errors::{IpcError, Result};
