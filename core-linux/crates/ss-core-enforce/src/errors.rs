use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnforceError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("cgroup: {0}")]
    Cgroup(String),
    #[error("dbus: {0}")]
    DBus(String),
    #[error("process gone: pid {0}")]
    ProcessGone(i32),
    #[error("store: {0}")]
    Store(#[from] ss_core_store::StoreError),
}

pub type Result<T> = std::result::Result<T, EnforceError>;
