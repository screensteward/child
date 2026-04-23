use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("invalid argon2id hash: {0}")]
    InvalidAuthHash(String),
    #[error("empty display name")]
    EmptyDisplayName,
    #[error("unsupported platform: {0}")]
    UnsupportedPlatform(String),
}
