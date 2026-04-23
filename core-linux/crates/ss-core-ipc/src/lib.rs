//! JSON-RPC 2.0 IPC server for `ScreenSteward` Core.

pub mod dto;
pub mod errors;
pub mod framing;
pub mod rpc;

pub use errors::{IpcError, Result};
