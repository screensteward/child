use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::dao::ts;
use crate::errors::Result;
use crate::store::Store;

/// Upserts the absolute minutes used for `(child_id, device_id, date)`.
///
/// The stored value is replaced, not incremented — callers must pass the
/// cumulative total for the day (G-Counter, single-device, Phase 1).
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn upsert_minutes(
    store: &Store,
    child_id: Uuid,
    device_id: Uuid,
    date: NaiveDate,
    minutes: u32,
) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "INSERT INTO usage_counter (child_id,device_id,date,minutes_used) VALUES (?1,?2,?3,?4)
             ON CONFLICT(child_id,device_id,date) DO UPDATE SET minutes_used = excluded.minutes_used",
            rusqlite::params![child_id.to_string(), device_id.to_string(), date.to_string(), minutes],
        )?;
        Ok(())
    })
}

/// Returns the total minutes used for a child on a given day (sum across all devices).
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
#[must_use = "returns total minutes; ignoring the result loses data"]
pub fn minutes_for_day(store: &Store, child_id: Uuid, date: NaiveDate) -> Result<u32> {
    store.with_conn(|c| {
        let sum: Option<i64> = c.query_row(
            "SELECT COALESCE(SUM(minutes_used),0) FROM usage_counter WHERE child_id=?1 AND date=?2",
            rusqlite::params![child_id.to_string(), date.to_string()],
            |r| r.get(0),
        )?;
        Ok(u32::try_from(sum.unwrap_or(0)).unwrap_or(u32::MAX))
    })
}

/// Records a single usage event (process launch / foreground window).
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
#[allow(clippy::too_many_arguments)]
pub fn record_event(
    store: &Store,
    child_id: Uuid,
    device_id: Uuid,
    content_hash: &str,
    basename: &str,
    path: &str,
    started_at: &DateTime<Utc>,
    ended_at: Option<&DateTime<Utc>>,
    category: Option<&str>,
) -> Result<Uuid> {
    let id = Uuid::new_v4();
    store.with_conn(|c| {
        c.execute(
            "INSERT INTO usage_event
             (id,child_id,device_id,content_hash,basename,path,started_at,ended_at,category)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            rusqlite::params![
                id.to_string(),
                child_id.to_string(),
                device_id.to_string(),
                content_hash,
                basename,
                path,
                ts(started_at),
                ended_at.map(ts),
                category,
            ],
        )?;
        Ok(())
    })?;
    Ok(id)
}

/// Returns the number of usage events recorded for a child across all devices.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn count_events_for_child(store: &Store, child_id: Uuid) -> Result<u64> {
    store.with_conn(|c| {
        let n: i64 = c.query_row(
            "SELECT COUNT(*) FROM usage_event WHERE child_id = ?1",
            rusqlite::params![child_id.to_string()],
            |r| r.get(0),
        )?;
        Ok(u64::try_from(n).unwrap_or(0))
    })
}
