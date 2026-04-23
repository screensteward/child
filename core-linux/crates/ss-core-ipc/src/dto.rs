//! DTO types for the IPC contract (mirrored on the Dart side via `json_serializable`).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStatus {
    pub version: String,
    pub uptime_seconds: u64,
    pub tpm_used: bool,
    pub db_ok: bool,
    pub last_enforcement_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildStatus {
    pub today_minutes_used: u32,
    pub today_budget_minutes: Option<u32>,
    pub current_window_open: bool,
    pub current_window_ends_at: Option<DateTime<Utc>>,
    pub active_blocklist_display: Vec<String>,
    pub session_running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReport {
    pub date: chrono::NaiveDate,
    pub usage_by_app: Vec<AppUsage>,
    pub total_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsage {
    pub content_hash: String,
    pub display_name: Option<String>,
    pub basename: String,
    pub minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilySnapshot {
    pub family_id: Uuid,
    pub family_name: String,
    pub parents: Vec<ParentView>,
    pub children: Vec<ChildView>,
    pub devices: Vec<DeviceView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentView {
    pub id: Uuid,
    pub display_name: String,
    // auth_hash is NEVER included here (security rule §8.4).
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildView {
    pub id: Uuid,
    pub display_name: String,
    pub birth_year: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceView {
    pub id: Uuid,
    pub child_id: Uuid,
    pub hostname: String,
    pub platform: String,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingState {
    Required,
    Complete,
}
