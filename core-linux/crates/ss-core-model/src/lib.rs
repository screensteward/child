//! Pure core model: entities, policies, matchers, evaluation.
//! No I/O, no tokio dep, no SQL. Testable without runtime.

pub mod entities;
pub mod errors;

pub use errors::ModelError;
