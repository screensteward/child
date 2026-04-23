use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::Result;
use crate::store::Store;

/// Status of a parent-granted screen-time exception.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExceptionStatus {
    Pending,
    Approved,
    Denied,
    Expired,
}

impl ExceptionStatus {
    /// Returns the canonical DB string for this status.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Denied => "denied",
            Self::Expired => "expired",
        }
    }
}

/// DAO-level representation of a `policy_exception` row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyException {
    pub id: Uuid,
    pub child_id: Uuid,
    pub granted_by_parent_id: Option<Uuid>,
    pub status: ExceptionStatus,
    pub reason: Option<String>,
    pub duration_minutes: Option<u32>,
    pub granted_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Inserts a new [`PolicyException`] row.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on constraint violation or I/O error.
pub fn insert(store: &Store, e: &PolicyException) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "INSERT INTO policy_exception
             (id,child_id,granted_by_parent_id,status,reason,duration_minutes,granted_at,expires_at,created_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            rusqlite::params![
                e.id.to_string(), e.child_id.to_string(),
                e.granted_by_parent_id.map(|u| u.to_string()),
                e.status.as_str(),
                e.reason.as_deref(),
                e.duration_minutes.map(i64::from),
                e.granted_at.as_ref().map(ts),
                e.expires_at.as_ref().map(ts),
                ts(&e.created_at),
            ],
        )?;
        Ok(())
    })
}

/// Returns all pending exceptions ordered by `created_at ASC`.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID or status string stored in the DB is not valid (invariant: written by us).
pub fn list_pending(store: &Store) -> Result<Vec<PolicyException>> {
    store.with_conn(|c| {
        let mut stmt = c.prepare(
            "SELECT id,child_id,granted_by_parent_id,status,reason,duration_minutes,granted_at,expires_at,created_at
             FROM policy_exception WHERE status = 'pending' ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map([], row_to_exception)?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    })
}

/// Updates the status (and optional approval fields) of a [`PolicyException`].
///
/// Fields passed as `None` are left unchanged in the DB via `COALESCE`.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn update_status(
    store: &Store,
    id: Uuid,
    status: &ExceptionStatus,
    granted_by_parent_id: Option<Uuid>,
    duration_minutes: Option<u32>,
    expires_at: Option<DateTime<Utc>>,
    granted_at: Option<DateTime<Utc>>,
) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "UPDATE policy_exception
             SET status = ?2,
                 granted_by_parent_id = COALESCE(?3, granted_by_parent_id),
                 duration_minutes = COALESCE(?4, duration_minutes),
                 expires_at = COALESCE(?5, expires_at),
                 granted_at = COALESCE(?6, granted_at)
             WHERE id = ?1",
            rusqlite::params![
                id.to_string(),
                status.as_str(),
                granted_by_parent_id.map(|u| u.to_string()),
                duration_minutes.map(i64::from),
                expires_at.as_ref().map(ts),
                granted_at.as_ref().map(ts),
            ],
        )?;
        Ok(())
    })
}

fn row_to_exception(row: &rusqlite::Row<'_>) -> rusqlite::Result<PolicyException> {
    let status_s: String = row.get(3)?;
    let status = match status_s.as_str() {
        "pending" => ExceptionStatus::Pending,
        "approved" => ExceptionStatus::Approved,
        "denied" => ExceptionStatus::Denied,
        "expired" => ExceptionStatus::Expired,
        other => panic!("invalid status in DB: {other}"),
    };
    Ok(PolicyException {
        id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
        child_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
        granted_by_parent_id: row
            .get::<_, Option<String>>(2)?
            .as_deref()
            .map(|s| Uuid::parse_str(s).unwrap()),
        status,
        reason: row.get(4)?,
        duration_minutes: row
            .get::<_, Option<i64>>(5)?
            .map(|v| u32::try_from(v).expect("invariant: duration_minutes fits in u32")),
        granted_at: row.get::<_, Option<String>>(6)?.as_deref().map(parse_ts),
        expires_at: row.get::<_, Option<String>>(7)?.as_deref().map(parse_ts),
        created_at: parse_ts(&row.get::<_, String>(8)?),
    })
}
