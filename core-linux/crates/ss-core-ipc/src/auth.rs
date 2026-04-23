//! Argon2id password hashing and per-connection auth state.
//!
//! The "token" for an authenticated IPC connection is the `authenticated_parent_id`
//! flag stored on the per-FD [`ConnState`]. Nothing is ever sent back to the client:
//! when the Unix socket closes, the `ConnState` is dropped and the authentication
//! is gone. This is a deliberate FD-scoped session design (see §8.3 Phase 1 spec).

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::errors::{IpcError, Result};

/// Hash a password with argon2id (OWASP 2025 defaults).
///
/// # Errors
///
/// Returns [`IpcError::Argon2`] if the underlying argon2 implementation
/// fails to hash (practically only on OOM or internal API misuse).
pub fn hash_password(plain: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let h = argon
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| IpcError::Argon2(e.to_string()))?
        .to_string();
    Ok(h)
}

/// Verify a plaintext password against a stored argon2id hash.
///
/// Returns `Ok(true)` on match, `Ok(false)` on mismatch.
///
/// # Errors
///
/// Returns [`IpcError::Argon2`] if `stored_hash` cannot be parsed as a
/// valid PHC-formatted argon2 hash.
pub fn verify_password(plain: &str, stored_hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(stored_hash).map_err(|e| IpcError::Argon2(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok())
}

/// Per-connection state holding the authentication flag and subscribed
/// notification topics. One instance lives in [`crate::server::handle_conn`]
/// and is dropped when the connection closes — so the "token" vanishes with
/// the socket FD.
#[derive(Debug, Clone, Default)]
pub struct ConnState {
    inner: Arc<RwLock<ConnStateInner>>,
}

#[derive(Debug, Default)]
struct ConnStateInner {
    authenticated_parent_id: Option<uuid::Uuid>,
    subscribed_topics: Vec<String>,
}

impl ConnState {
    /// Fresh, unauthenticated, no-subscriptions state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark this connection as authenticated for the given parent.
    pub fn authenticate(&self, parent_id: uuid::Uuid) {
        self.inner.write().authenticated_parent_id = Some(parent_id);
    }

    /// Drop the authentication flag.
    pub fn logout(&self) {
        self.inner.write().authenticated_parent_id = None;
    }

    /// Whether this connection is currently authenticated.
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        self.inner.read().authenticated_parent_id.is_some()
    }

    /// The parent identity bound to this connection, if any.
    #[must_use]
    pub fn parent_id(&self) -> Option<uuid::Uuid> {
        self.inner.read().authenticated_parent_id
    }

    /// Add the given topics to the subscription set (deduplicated).
    pub fn subscribe(&self, topics: Vec<String>) {
        let mut g = self.inner.write();
        for t in topics {
            if !g.subscribed_topics.contains(&t) {
                g.subscribed_topics.push(t);
            }
        }
    }

    /// Whether the connection is subscribed to `topic`.
    #[must_use]
    pub fn is_subscribed(&self, topic: &str) -> bool {
        self.inner
            .read()
            .subscribed_topics
            .iter()
            .any(|t| t == topic)
    }
}
