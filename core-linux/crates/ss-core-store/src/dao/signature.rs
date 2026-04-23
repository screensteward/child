use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dao::{parse_ts, ts};
use crate::errors::Result;
use crate::store::Store;

/// DAO-level representation of an `app_signature` row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSignature {
    pub content_hash: String,
    pub display_name: Option<String>,
    pub known_basenames: Vec<String>,
    pub known_paths: Vec<String>,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

/// Records an observation of `(content_hash, basename, path)`, creating or updating
/// the `app_signature` row. Deduplicates `known_basenames` and `known_paths`.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] or [`crate::errors::StoreError::Json`] on error.
pub fn upsert_observation(
    store: &Store,
    content_hash: &str,
    basename: &str,
    path: &str,
) -> Result<()> {
    let now = Utc::now();
    store.with_conn(|c| {
        let existing: Option<(String, String)> = c
            .query_row(
                "SELECT known_basenames, known_paths FROM app_signature WHERE content_hash = ?1",
                rusqlite::params![content_hash],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();

        match existing {
            Some((b_json, p_json)) => {
                let mut basenames: Vec<String> =
                    serde_json::from_str(&b_json).unwrap_or_default();
                let mut paths: Vec<String> = serde_json::from_str(&p_json).unwrap_or_default();
                if !basenames.iter().any(|x| x == basename) {
                    basenames.push(basename.into());
                }
                if !paths.iter().any(|x| x == path) {
                    paths.push(path.into());
                }
                c.execute(
                    "UPDATE app_signature
                     SET known_basenames = ?2, known_paths = ?3, last_seen_at = ?4
                     WHERE content_hash = ?1",
                    rusqlite::params![
                        content_hash,
                        serde_json::to_string(&basenames)?,
                        serde_json::to_string(&paths)?,
                        ts(&now),
                    ],
                )?;
            }
            None => {
                c.execute(
                    "INSERT INTO app_signature
                     (content_hash, display_name, known_basenames, known_paths, first_seen_at, last_seen_at)
                     VALUES (?1, NULL, ?2, ?3, ?4, ?4)",
                    rusqlite::params![
                        content_hash,
                        serde_json::to_string(&vec![basename.to_string()])?,
                        serde_json::to_string(&vec![path.to_string()])?,
                        ts(&now),
                    ],
                )?;
            }
        }
        Ok(())
    })
}

/// Returns the [`AppSignature`] for `content_hash`, or `None` if not found.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn get(store: &Store, content_hash: &str) -> Result<Option<AppSignature>> {
    store.with_conn(|c| {
        let row = c.query_row(
            "SELECT content_hash,display_name,known_basenames,known_paths,first_seen_at,last_seen_at
             FROM app_signature WHERE content_hash = ?1",
            rusqlite::params![content_hash],
            |r| {
                Ok(AppSignature {
                    content_hash: r.get(0)?,
                    display_name: r.get(1)?,
                    known_basenames: serde_json::from_str::<Vec<String>>(
                        &r.get::<_, String>(2)?,
                    )
                    .unwrap_or_default(),
                    known_paths: serde_json::from_str::<Vec<String>>(&r.get::<_, String>(3)?)
                        .unwrap_or_default(),
                    first_seen_at: parse_ts(&r.get::<_, String>(4)?),
                    last_seen_at: parse_ts(&r.get::<_, String>(5)?),
                })
            },
        );
        match row {
            Ok(s) => Ok(Some(s)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    })
}

/// Returns all [`AppSignature`]s whose `known_basenames` contains `basename`
/// (case-insensitive ASCII comparison).
///
/// Phase 1 implementation loads all rows and filters in Rust — acceptable at MVP scale.
///
/// # Errors
///
/// Returns [`crate::errors::StoreError::Sqlite`] on I/O error.
pub fn find_by_basename(store: &Store, basename: &str) -> Result<Vec<AppSignature>> {
    store.with_conn(|c| {
        let mut stmt = c.prepare(
            "SELECT content_hash,display_name,known_basenames,known_paths,first_seen_at,last_seen_at
             FROM app_signature",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(AppSignature {
                content_hash: r.get(0)?,
                display_name: r.get(1)?,
                known_basenames: serde_json::from_str::<Vec<String>>(&r.get::<_, String>(2)?)
                    .unwrap_or_default(),
                known_paths: serde_json::from_str::<Vec<String>>(&r.get::<_, String>(3)?)
                    .unwrap_or_default(),
                first_seen_at: parse_ts(&r.get::<_, String>(4)?),
                last_seen_at: parse_ts(&r.get::<_, String>(5)?),
            })
        })?;
        let all: Vec<_> = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(all
            .into_iter()
            .filter(|s| {
                s.known_basenames
                    .iter()
                    .any(|b| b.eq_ignore_ascii_case(basename))
            })
            .collect())
    })
}
