use chrono::{TimeZone, Utc};
use ss_core_model::entities::{Child, ChildDevice, Family, Parent, Platform};
use uuid::Uuid;

#[test]
fn family_roundtrip_json() {
    let family = Family {
        id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        name: "Test".into(),
        created_at: Utc.with_ymd_and_hms(2026, 4, 23, 12, 0, 0).unwrap(),
        modified_at: Utc.with_ymd_and_hms(2026, 4, 23, 12, 0, 0).unwrap(),
    };
    let s = serde_json::to_string(&family).unwrap();
    let back: Family = serde_json::from_str(&s).unwrap();
    assert_eq!(family, back);
}

#[test]
fn parent_requires_auth_hash() {
    let p = Parent {
        id: Uuid::new_v4(),
        family_id: Uuid::new_v4(),
        display_name: "Alice".into(),
        auth_hash: "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$hash".into(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    assert!(p.auth_hash.starts_with("$argon2id$"));
}

#[test]
fn child_device_platform_serializes_lowercase() {
    let d = ChildDevice {
        id: Uuid::new_v4(),
        child_id: Uuid::new_v4(),
        hostname: "workstation".into(),
        platform: Platform::Linux,
        last_seen_at: Utc::now(),
    };
    let s = serde_json::to_string(&d).unwrap();
    assert!(s.contains("\"platform\":\"linux\""));
}

#[test]
fn child_birth_year_optional() {
    let c = Child {
        id: Uuid::new_v4(),
        family_id: Uuid::new_v4(),
        display_name: "Bob".into(),
        birth_year: None,
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    let s = serde_json::to_string(&c).unwrap();
    assert!(s.contains("\"birth_year\":null"));
}
