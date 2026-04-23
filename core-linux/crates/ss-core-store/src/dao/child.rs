use ss_core_model::entities::Child;
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::Result;
use crate::store::Store;

/// Inserts a [`Child`] row.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on constraint violation or I/O error.
pub fn insert(store: &Store, c: &Child) -> Result<()> {
    store.with_conn(|conn| {
        conn.execute(
            "INSERT INTO child (id,family_id,display_name,birth_year,created_at,modified_at)
             VALUES (?1,?2,?3,?4,?5,?6)",
            rusqlite::params![
                c.id.to_string(),
                c.family_id.to_string(),
                c.display_name,
                c.birth_year.map(i64::from),
                ts(&c.created_at),
                ts(&c.modified_at),
            ],
        )?;
        Ok(())
    })
}

/// Returns all [`Child`] rows for the given `family_id`, ordered by `created_at`.
///
/// # Errors
///
/// Returns [`StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID or `birth_year` stored in the DB is not valid
/// (invariant: written by us, birth years fit in u16).
pub fn list_by_family(store: &Store, family_id: Uuid) -> Result<Vec<Child>> {
    store.with_conn(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id,family_id,display_name,birth_year,created_at,modified_at
             FROM child WHERE family_id = ?1 ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map(rusqlite::params![family_id.to_string()], |row| {
            Ok(Child {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                family_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                display_name: row.get(2)?,
                birth_year: row
                    .get::<_, Option<i64>>(3)?
                    .map(|v| u16::try_from(v).expect("invariant: birth_year fits in u16")),
                created_at: parse_ts(&row.get::<_, String>(4)?),
                modified_at: parse_ts(&row.get::<_, String>(5)?),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    })
}
