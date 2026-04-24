use ss_core_model::entities::Family;
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::{Result, StoreError};
use crate::store::Store;

/// Inserts a [`Family`] row.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on constraint violation or I/O error.
pub fn insert(store: &Store, f: &Family) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "INSERT INTO family (id,name,created_at,modified_at) VALUES (?1,?2,?3,?4)",
            rusqlite::params![
                f.id.to_string(),
                f.name,
                ts(&f.created_at),
                ts(&f.modified_at)
            ],
        )?;
        Ok(())
    })
}

/// Returns the [`Family`] with the given `id`.
///
/// # Errors
///
/// Returns [`StoreError::NotFound`] if no row matches, or [`StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if the UUID stored in the DB is not valid (invariant: written by us).
pub fn get(store: &Store, id: Uuid) -> Result<Family> {
    store.with_conn(|c| {
        c.query_row(
            "SELECT id,name,created_at,modified_at FROM family WHERE id = ?1",
            rusqlite::params![id.to_string()],
            |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let c_at: String = row.get(2)?;
                let m_at: String = row.get(3)?;
                Ok((id, name, c_at, m_at))
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => StoreError::NotFound,
            other => StoreError::Sqlite(other),
        })
        .map(|(id, name, c_at, m_at)| Family {
            id: Uuid::parse_str(&id).unwrap(),
            name,
            created_at: parse_ts(&c_at),
            modified_at: parse_ts(&m_at),
        })
    })
}

/// Returns the first [`Family`] row, or `None` if the table is empty.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if the UUID stored in the DB is not valid (invariant: written by us).
pub fn get_single(store: &Store) -> Result<Option<Family>> {
    store.with_conn(|c| {
        let mut stmt = c.prepare("SELECT id,name,created_at,modified_at FROM family LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let c_at: String = row.get(2)?;
            let m_at: String = row.get(3)?;
            Ok(Family {
                id: Uuid::parse_str(&id).unwrap(),
                name,
                created_at: parse_ts(&c_at),
                modified_at: parse_ts(&m_at),
            })
        });
        match row {
            Ok(f) => Ok(Some(f)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StoreError::Sqlite(e)),
        }
    })
}
