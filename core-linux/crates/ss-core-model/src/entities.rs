use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parent {
    pub id: Uuid,
    pub family_id: Uuid,
    pub display_name: String,
    /// argon2id encoded hash (format `$argon2id$v=19$...`).
    pub auth_hash: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Child {
    pub id: Uuid,
    pub family_id: Uuid,
    pub display_name: String,
    pub birth_year: Option<u16>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Linux,
    #[serde(rename = "macos")]
    MacOs,
    Windows,
    Android,
    Ios,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildDevice {
    pub id: Uuid,
    pub child_id: Uuid,
    pub hostname: String,
    pub platform: Platform,
    pub last_seen_at: DateTime<Utc>,
}
