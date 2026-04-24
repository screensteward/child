use chrono::{DateTime, Utc};
use ss_core_model::policy::{Policy, Rule, Scope};
use uuid::Uuid;

use crate::dao::{parse_ts, ts};
use crate::errors::Result;
use crate::store::Store;

/// Inserts a [`Policy`] row.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on constraint violation or I/O error.
pub fn insert(store: &Store, p: &Policy) -> Result<()> {
    let scope_json = serde_json::to_string(&p.scope)?;
    let rules_json = serde_json::to_string(&p.rules)?;
    store.with_conn(|c| {
        c.execute(
            "INSERT INTO policy (id,child_id,scope_json,rules_json,priority,active_from,active_until,created_at,modified_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            rusqlite::params![
                p.id.to_string(), p.child_id.to_string(),
                scope_json, rules_json, p.priority,
                p.active_from.as_ref().map(ts),
                p.active_until.as_ref().map(ts),
                ts(&p.created_at), ts(&p.modified_at),
            ],
        )?;
        Ok(())
    })
}

/// Updates scope, rules, priority and validity window for an existing [`Policy`].
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn update(store: &Store, p: &Policy) -> Result<()> {
    let scope_json = serde_json::to_string(&p.scope)?;
    let rules_json = serde_json::to_string(&p.rules)?;
    store.with_conn(|c| {
        c.execute(
            "UPDATE policy SET scope_json=?2,rules_json=?3,priority=?4,active_from=?5,active_until=?6,modified_at=?7
             WHERE id = ?1",
            rusqlite::params![
                p.id.to_string(), scope_json, rules_json, p.priority,
                p.active_from.as_ref().map(ts),
                p.active_until.as_ref().map(ts),
                ts(&p.modified_at),
            ],
        )?;
        Ok(())
    })
}

/// Deletes the [`Policy`] with the given `id`.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn delete(store: &Store, id: Uuid) -> Result<()> {
    store.with_conn(|c| {
        c.execute(
            "DELETE FROM policy WHERE id = ?1",
            rusqlite::params![id.to_string()],
        )?;
        Ok(())
    })
}

/// Returns all policies for a child, ordered by `priority DESC, created_at ASC`.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID or JSON stored in the DB is not valid (invariant: written by us).
pub fn list_by_child(store: &Store, child_id: Uuid) -> Result<Vec<Policy>> {
    store.with_conn(|c| {
        let mut stmt = c.prepare(
            "SELECT id,child_id,scope_json,rules_json,priority,active_from,active_until,created_at,modified_at
             FROM policy WHERE child_id = ?1 ORDER BY priority DESC, created_at ASC",
        )?;
        let rows = stmt.query_map(rusqlite::params![child_id.to_string()], row_to_policy)?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    })
}

/// Returns policies for a child that are active at `now` (within `active_from`/`active_until` window).
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
///
/// # Panics
///
/// Panics if a UUID or JSON stored in the DB is not valid (invariant: written by us).
pub fn list_active(store: &Store, child_id: Uuid, now: &DateTime<Utc>) -> Result<Vec<Policy>> {
    let all = list_by_child(store, child_id)?;
    Ok(all
        .into_iter()
        .filter(|p| {
            p.active_from.map_or(true, |t| t <= *now) && p.active_until.map_or(true, |t| t > *now)
        })
        .collect())
}

fn row_to_policy(row: &rusqlite::Row<'_>) -> rusqlite::Result<Policy> {
    let scope_s: String = row.get(2)?;
    let rules_s: String = row.get(3)?;
    let scope: Scope = serde_json::from_str(&scope_s).expect("invariant: scope JSON");
    let rules: Vec<Rule> = serde_json::from_str(&rules_s).expect("invariant: rules JSON");
    Ok(Policy {
        id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
        child_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
        scope,
        rules,
        priority: row.get(4)?,
        active_from: row.get::<_, Option<String>>(5)?.as_deref().map(parse_ts),
        active_until: row.get::<_, Option<String>>(6)?.as_deref().map(parse_ts),
        created_at: parse_ts(&row.get::<_, String>(7)?),
        modified_at: parse_ts(&row.get::<_, String>(8)?),
    })
}
