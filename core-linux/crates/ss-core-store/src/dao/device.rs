use ss_core_model::entities::{ChildDevice, Platform};
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::Result;
use crate::store::Store;

fn platform_to_str(p: Platform) -> &'static str {
    match p {
        Platform::Linux => "linux",
        Platform::MacOs => "macos",
        Platform::Windows => "windows",
        Platform::Android => "android",
        Platform::Ios => "ios",
    }
}

fn platform_from_str(s: &str) -> Platform {
    match s {
        "linux" => Platform::Linux,
        "macos" => Platform::MacOs,
        "windows" => Platform::Windows,
        "android" => Platform::Android,
        "ios" => Platform::Ios,
        other => panic!("unknown platform in DB: {other}"),
    }
}

/// Inserts a [`ChildDevice`] row.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on constraint violation or I/O error.
pub fn insert(store: &Store, d: &ChildDevice) -> Result<()> {
    store.with_conn(|conn| {
        conn.execute(
            "INSERT INTO child_device (id,child_id,hostname,platform,noise_pubkey,last_seen_at)
             VALUES (?1,?2,?3,?4,NULL,?5)",
            rusqlite::params![
                d.id.to_string(),
                d.child_id.to_string(),
                d.hostname,
                platform_to_str(d.platform),
                ts(&d.last_seen_at),
            ],
        )?;
        Ok(())
    })
}

/// Returns all [`ChildDevice`] rows for the given `child_id`.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID or platform string stored in the DB is not valid
/// (invariant: written by us).
pub fn list_by_child(store: &Store, child_id: Uuid) -> Result<Vec<ChildDevice>> {
    store.with_conn(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id,child_id,hostname,platform,last_seen_at
             FROM child_device WHERE child_id = ?1",
        )?;
        let rows = stmt.query_map(rusqlite::params![child_id.to_string()], |row| {
            Ok(ChildDevice {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                child_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                hostname: row.get(2)?,
                platform: platform_from_str(&row.get::<_, String>(3)?),
                last_seen_at: parse_ts(&row.get::<_, String>(4)?),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    })
}

/// Updates `last_seen_at` for the given `device_id` to now.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on I/O error.
pub fn touch_last_seen(store: &Store, device_id: Uuid) -> Result<()> {
    let now = ts(&chrono::Utc::now());
    store.with_conn(|conn| {
        conn.execute(
            "UPDATE child_device SET last_seen_at = ?2 WHERE id = ?1",
            rusqlite::params![device_id.to_string(), now],
        )?;
        Ok(())
    })
}
