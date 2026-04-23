use chrono::{NaiveDate, Utc};
use ss_core_model::entities::{Child, ChildDevice, Family, Platform};
use ss_core_store::{dao, Store};
use tempfile::tempdir;
use uuid::Uuid;

fn seed() -> (tempfile::TempDir, Store, Uuid, Uuid) {
    let d = tempdir().unwrap();
    let s = Store::open_with_key(&d.path().join("c.db"), &[0u8; 32]).unwrap();
    let fam_id = Uuid::new_v4();
    dao::family::insert(&s, &Family { id: fam_id, name: "F".into(), created_at: Utc::now(), modified_at: Utc::now() }).unwrap();
    let child_id = Uuid::new_v4();
    dao::child::insert(&s, &Child { id: child_id, family_id: fam_id, display_name: "B".into(), birth_year: None, created_at: Utc::now(), modified_at: Utc::now() }).unwrap();
    let device_id = Uuid::new_v4();
    dao::device::insert(&s, &ChildDevice { id: device_id, child_id, hostname: "h".into(), platform: Platform::Linux, last_seen_at: Utc::now() }).unwrap();
    (d, s, child_id, device_id)
}

#[test]
fn usage_counter_increments_idempotent_per_day() {
    let (_d, s, child_id, device_id) = seed();
    let date = NaiveDate::from_ymd_opt(2026, 4, 23).unwrap();
    dao::usage::upsert_minutes(&s, child_id, device_id, date, 10).unwrap();
    dao::usage::upsert_minutes(&s, child_id, device_id, date, 25).unwrap();
    let t = dao::usage::minutes_for_day(&s, child_id, date).unwrap();
    assert_eq!(t, 25, "upsert stores absolute value, not increment");
}

#[test]
fn usage_events_recorded() {
    let (_d, s, child_id, device_id) = seed();
    dao::usage::record_event(
        &s, child_id, device_id,
        "sha256:abc", "steam", "/usr/bin/steam",
        &Utc::now(), None, None,
    ).unwrap();
    let n = dao::usage::count_events_for_child(&s, child_id).unwrap();
    assert_eq!(n, 1);
}
