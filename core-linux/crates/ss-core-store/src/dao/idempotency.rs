use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::Result;
use crate::store::Store;

/// Returns the cached `response_json` for `request_id`, or `None` if not found.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn get(store: &Store, request_id: Uuid) -> Result<Option<String>> {
    store.with_conn(|c| {
        let row = c.query_row(
            "SELECT response_json FROM idempotency WHERE request_id = ?1",
            rusqlite::params![request_id.to_string()],
            |r| r.get::<_, String>(0),
        );
        match row {
            Ok(s) => Ok(Some(s)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    })
}

/// Stores an idempotency entry. No-op if `request_id` already exists.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn put(
    store: &Store,
    request_id: Uuid,
    response_json: &str,
    seen_at: &DateTime<Utc>,
) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "INSERT OR IGNORE INTO idempotency (request_id,response_json,seen_at)
             VALUES (?1,?2,?3)",
            rusqlite::params![request_id.to_string(), response_json, ts(seen_at)],
        )?;
        Ok(())
    })
}

/// Deletes entries whose `seen_at` is older than `ttl` relative to `now`.
///
/// Returns the number of rows deleted.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn purge_older_than(store: &Store, ttl: Duration, now: &DateTime<Utc>) -> Result<u64> {
    let cutoff = *now - ttl;
    store.with_conn(|c| {
        let n = c.execute(
            "DELETE FROM idempotency WHERE seen_at < ?1",
            rusqlite::params![ts(&cutoff)],
        )?;
        Ok(u64::try_from(n).unwrap_or(0))
    })
}

/// Parses an RFC-3339 timestamp string — used only in tests.
///
/// # Panics
///
/// Panics if `s` is not a valid RFC-3339 timestamp.
#[doc(hidden)]
#[must_use]
pub fn parse_seen_at(s: &str) -> DateTime<Utc> {
    parse_ts(s)
}
