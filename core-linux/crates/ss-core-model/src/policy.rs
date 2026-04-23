use chrono::{DateTime, Datelike, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Child,
    /// Réservé Phase 2+.
    Device { device_id: Uuid },
    /// Réservé Phase 3+.
    Category { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppMatcher {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_glob: Option<String>,
}

impl AppMatcher {
    /// Un process (`hash`, `basename`, `path`) match si au moins un des
    /// champs renseignés du matcher match. Les champs `None` sont ignorés.
    ///
    /// `hash` est `Option<&str>` pour gérer le cas où le hash n'a pas encore
    /// été calculé (process très récent, file unreadable) — l'absence de
    /// hash n'empêche pas un match par `basename` / `path_glob`.
    #[must_use]
    pub fn matches(&self, hash: Option<&str>, basename: &str, path: &str) -> bool {
        if let Some(expected) = &self.content_hash {
            if hash.is_some_and(|h| h == expected) {
                return true;
            }
        }
        if let Some(b) = &self.basename {
            if b.eq_ignore_ascii_case(basename) {
                return true;
            }
        }
        if let Some(p) = &self.path_glob {
            if glob_match(p, path) {
                return true;
            }
        }
        false
    }
}

/// Implémentation minimale glob : `*` = n'importe quoi sauf `/`, `**` = récursif, `?` = un char.
/// Suffisant pour les patterns de type `/home/**/steam` et `/usr/bin/*`.
fn glob_match(pattern: &str, path: &str) -> bool {
    let p_bytes = pattern.as_bytes();
    let s_bytes = path.as_bytes();
    glob_recurse(p_bytes, 0, s_bytes, 0)
}

fn glob_recurse(p: &[u8], pi: usize, s: &[u8], si: usize) -> bool {
    if pi == p.len() {
        return si == s.len();
    }
    match p[pi] {
        b'*' if pi + 1 < p.len() && p[pi + 1] == b'*' => {
            // **  — match récursif
            for k in si..=s.len() {
                if glob_recurse(p, pi + 2, s, k) {
                    return true;
                }
            }
            false
        }
        b'*' => {
            // * — match tout sauf '/'
            for k in si..=s.len() {
                if s[si..k].contains(&b'/') {
                    break;
                }
                if glob_recurse(p, pi + 1, s, k) {
                    return true;
                }
            }
            false
        }
        b'?' => si < s.len() && s[si] != b'/' && glob_recurse(p, pi + 1, s, si + 1),
        c => si < s.len() && s[si] == c && glob_recurse(p, pi + 1, s, si + 1),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyBudget {
    pub minutes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Jours actifs : 1=lundi … 7=dimanche (ISO 8601).
    pub days: Vec<u8>,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeWindow {
    #[must_use]
    pub fn is_open(&self, at: &DateTime<Utc>) -> bool {
        // number_from_monday() returns 1..=7, fits in u8 safely
        #[allow(clippy::cast_possible_truncation)]
        let weekday = at.weekday().number_from_monday() as u8;
        if !self.days.contains(&weekday) {
            return false;
        }
        let t = at.time();
        t >= self.start && t < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rule {
    DailyBudget(DailyBudget),
    TimeWindow(TimeWindow),
    AppBlocklist { matchers: Vec<AppMatcher> },
    AppAllowlist { matchers: Vec<AppMatcher> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub child_id: Uuid,
    pub scope: Scope,
    pub rules: Vec<Rule>,
    pub priority: i32,
    pub active_from: Option<DateTime<Utc>>,
    pub active_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
