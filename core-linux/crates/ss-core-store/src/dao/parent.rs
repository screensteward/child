use ss_core_model::entities::Parent;
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::{Result, StoreError};
use crate::store::Store;

/// Inserts a [`Parent`] row.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on constraint violation or I/O error.
pub fn insert(store: &Store, p: &Parent) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "INSERT INTO parent (id,family_id,display_name,auth_hash,created_at,modified_at)
             VALUES (?1,?2,?3,?4,?5,?6)",
            rusqlite::params![
                p.id.to_string(),
                p.family_id.to_string(),
                p.display_name,
                p.auth_hash,
                ts(&p.created_at),
                ts(&p.modified_at),
            ],
        )?;
        Ok(())
    })
}

/// Returns the [`Parent`] with the given `id`.
///
/// # Errors
///
/// Returns [`StoreError::NotFound`] if no row matches, or [`StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID stored in the DB is not valid (invariant: written by us).
pub fn get(store: &Store, id: Uuid) -> Result<Parent> {
    store.with_conn(|c| {
        c.query_row(
            "SELECT id,family_id,display_name,auth_hash,created_at,modified_at
             FROM parent WHERE id = ?1",
            rusqlite::params![id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                ))
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => StoreError::NotFound,
            other => StoreError::Sqlite(other),
        })
        .map(|(id, fam, name, hash, c_at, m_at)| Parent {
            id: Uuid::parse_str(&id).unwrap(),
            family_id: Uuid::parse_str(&fam).unwrap(),
            display_name: name,
            auth_hash: hash,
            created_at: parse_ts(&c_at),
            modified_at: parse_ts(&m_at),
        })
    })
}

/// Returns the first [`Parent`] row, or `None` if the table is empty.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID stored in the DB is not valid (invariant: written by us).
pub fn get_single(store: &Store) -> Result<Option<Parent>> {
    store.with_conn(|c| {
        let mut stmt = c.prepare(
            "SELECT id,family_id,display_name,auth_hash,created_at,modified_at
             FROM parent LIMIT 1",
        )?;
        let row = stmt.query_row([], |row| {
            Ok(Parent {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                family_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                display_name: row.get(2)?,
                auth_hash: row.get(3)?,
                created_at: parse_ts(&row.get::<_, String>(4)?),
                modified_at: parse_ts(&row.get::<_, String>(5)?),
            })
        });
        match row {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StoreError::Sqlite(e)),
        }
    })
}

/// Updates the `auth_hash` and `modified_at` for the given `parent_id`.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on I/O error.
pub fn update_password_hash(store: &Store, parent_id: Uuid, hash: &str) -> Result<()> {
    let now = ts(&chrono::Utc::now());
    store.with_conn(|c| {
        c.execute(
            "UPDATE parent SET auth_hash = ?2, modified_at = ?3 WHERE id = ?1",
            rusqlite::params![parent_id.to_string(), hash, now],
        )?;
        Ok(())
    })
}
