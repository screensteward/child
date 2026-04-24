use chrono::Utc;
use ss_core_model::entities::{Child, ChildDevice, Family, Parent, Platform};
use ss_core_store::Store;
use tempfile::tempdir;
use uuid::Uuid;

fn open() -> (tempfile::TempDir, Store) {
    let d = tempdir().unwrap();
    let p = d.path().join("core.db");
    let store = Store::open_with_key(&p, &[0u8; 32]).unwrap();
    (d, store)
}

#[test]
fn family_insert_and_get() {
    let (_d, s) = open();
    let fam = Family {
        id: Uuid::new_v4(),
        name: "Foux".into(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    ss_core_store::dao::family::insert(&s, &fam).unwrap();
    let back = ss_core_store::dao::family::get(&s, fam.id).unwrap();
    assert_eq!(back.name, "Foux");
}

#[test]
fn family_get_missing_returns_not_found() {
    let (_d, s) = open();
    let err = ss_core_store::dao::family::get(&s, Uuid::new_v4()).unwrap_err();
    assert!(matches!(err, ss_core_store::StoreError::NotFound));
}

#[test]
fn parent_and_child_crud() {
    let (_d, s) = open();
    let fam_id = Uuid::new_v4();
    ss_core_store::dao::family::insert(
        &s,
        &Family {
            id: fam_id,
            name: "F".into(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        },
    )
    .unwrap();
    let p = Parent {
        id: Uuid::new_v4(),
        family_id: fam_id,
        display_name: "Alice".into(),
        auth_hash: "$argon2id$v=19$...".into(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    ss_core_store::dao::parent::insert(&s, &p).unwrap();
    let got = ss_core_store::dao::parent::get(&s, p.id).unwrap();
    assert_eq!(got.display_name, "Alice");
    let c = Child {
        id: Uuid::new_v4(),
        family_id: fam_id,
        display_name: "Bob".into(),
        birth_year: Some(2015),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    ss_core_store::dao::child::insert(&s, &c).unwrap();
    let by_family = ss_core_store::dao::child::list_by_family(&s, fam_id).unwrap();
    assert_eq!(by_family.len(), 1);
    assert_eq!(by_family[0].birth_year, Some(2015));
}

#[test]
fn device_crud() {
    let (_d, s) = open();
    let fam_id = Uuid::new_v4();
    let child_id = Uuid::new_v4();
    ss_core_store::dao::family::insert(
        &s,
        &Family {
            id: fam_id,
            name: "F".into(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        },
    )
    .unwrap();
    ss_core_store::dao::child::insert(
        &s,
        &Child {
            id: child_id,
            family_id: fam_id,
            display_name: "Bob".into(),
            birth_year: None,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        },
    )
    .unwrap();
    let d = ChildDevice {
        id: Uuid::new_v4(),
        child_id,
        hostname: "workstation".into(),
        platform: Platform::Linux,
        last_seen_at: Utc::now(),
    };
    ss_core_store::dao::device::insert(&s, &d).unwrap();
    let list = ss_core_store::dao::device::list_by_child(&s, child_id).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].platform, Platform::Linux);
}
