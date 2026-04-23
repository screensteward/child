//! JSON-RPC 2.0 IPC server for `ScreenSteward` Core.

pub mod auth;
pub mod dto;
pub mod errors;
pub mod framing;
pub mod idempotency;
pub mod rpc;
pub mod server;

pub use errors::{IpcError, Result};
