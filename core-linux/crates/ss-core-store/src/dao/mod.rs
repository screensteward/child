//! DAO par entité — conversions rusqlite ↔ `ss_core_model`.

pub mod child;
pub mod device;
pub mod family;
pub mod parent;

use chrono::{DateTime, Utc};

pub(crate) fn ts(t: &DateTime<Utc>) -> String {
    t.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

pub(crate) fn parse_ts(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .expect("invariant: rfc3339 stored by us")
        .with_timezone(&Utc)
}
