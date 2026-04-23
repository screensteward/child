//! Persistance `SQLCipher` pour `ScreenSteward` Core.

pub mod errors;
pub mod store;

pub use errors::{Result, StoreError};
pub use store::Store;
