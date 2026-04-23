//! RAM-only idempotency cache (TTL 1h by default).
//!
//! A small in-memory buffer is enough for the IPC server. Persistent storage
//! is an optional alternative provided by `ss-core-store::dao::idempotency`.

use chrono::{DateTime, Duration, Utc};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

type Entry = (serde_json::Value, DateTime<Utc>);

/// In-memory cache of JSON-RPC results keyed by idempotency id.
#[derive(Debug, Clone)]
pub struct IdempotencyCache {
    inner: Arc<Mutex<HashMap<Uuid, Entry>>>,
    ttl: Duration,
}

impl IdempotencyCache {
    /// Build an empty cache with the given TTL.
    #[must_use]
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    /// Return the cached value for `id` if still fresh; evict it in-line
    /// if expired.
    #[must_use]
    pub fn get(&self, id: Uuid) -> Option<serde_json::Value> {
        let mut g = self.inner.lock();
        if let Some((v, t)) = g.get(&id) {
            if Utc::now() - *t < self.ttl {
                return Some(v.clone());
            }
            g.remove(&id);
        }
        None
    }

    /// Store `v` under `id` with "now" as the timestamp.
    pub fn put(&self, id: Uuid, v: serde_json::Value) {
        self.inner.lock().insert(id, (v, Utc::now()));
    }

    /// Drop every entry whose timestamp is older than `ttl`. Returns the
    /// number of evicted entries.
    #[must_use]
    pub fn purge_expired(&self) -> usize {
        let mut g = self.inner.lock();
        let now = Utc::now();
        let before = g.len();
        g.retain(|_, (_, t)| now - *t < self.ttl);
        before - g.len()
    }
}
