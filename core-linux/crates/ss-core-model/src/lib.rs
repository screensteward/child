//! Modèle pur du Core : entités, policies, matchers, évaluation, compteur CRDT.

pub mod counter;
pub mod entities;
pub mod errors;
pub mod evaluator;
pub mod policy;

pub use errors::ModelError;
