use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("framing: {0}")]
    Framing(String),
    #[error("rpc: {0}")]
    Rpc(String),
    #[error("method not found: {0}")]
    MethodNotFound(String),
    #[error("invalid params: {0}")]
    InvalidParams(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("argon2: {0}")]
    Argon2(String),
    #[error("store: {0}")]
    Store(#[from] ss_core_store::StoreError),
}

pub type Result<T> = std::result::Result<T, IpcError>;
