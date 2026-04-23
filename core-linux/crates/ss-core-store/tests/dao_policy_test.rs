use chrono::Utc;
use ss_core_model::entities::{Child, Family};
use ss_core_model::policy::{DailyBudget, Policy, Rule, Scope};
use ss_core_store::dao;
use ss_core_store::Store;
use tempfile::tempdir;
use uuid::Uuid;

fn seed() -> (tempfile::TempDir, Store, Uuid) {
    let d = tempdir().unwrap();
    let s = Store::open_with_key(&d.path().join("c.db"), &[0u8; 32]).unwrap();
    let fam_id = Uuid::new_v4();
    dao::family::insert(&s, &Family {
        id: fam_id, name: "F".into(),
        created_at: Utc::now(), modified_at: Utc::now(),
    }).unwrap();
    let child_id = Uuid::new_v4();
    dao::child::insert(&s, &Child {
        id: child_id, family_id: fam_id,
        display_name: "B".into(), birth_year: None,
        created_at: Utc::now(), modified_at: Utc::now(),
    }).unwrap();
    (d, s, child_id)
}

#[test]
fn policy_crud_and_list() {
    let (_d, s, child_id) = seed();
    let p = Policy {
        id: Uuid::new_v4(),
        child_id,
        scope: Scope::Child,
        rules: vec![Rule::DailyBudget(DailyBudget { minutes: 120 })],
        priority: 0,
        active_from: None,
        active_until: None,
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    dao::policy::insert(&s, &p).unwrap();
    let list = dao::policy::list_by_child(&s, child_id).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, p.id);
    dao::policy::delete(&s, p.id).unwrap();
    let list2 = dao::policy::list_by_child(&s, child_id).unwrap();
    assert!(list2.is_empty());
}

#[test]
fn policy_all_active_returns_only_active() {
    let (_d, s, child_id) = seed();
    let now = Utc::now();
    let active = Policy {
        id: Uuid::new_v4(), child_id, scope: Scope::Child,
        rules: vec![], priority: 0,
        active_from: None, active_until: None,
        created_at: now, modified_at: now,
    };
    let expired = Policy {
        id: Uuid::new_v4(), child_id, scope: Scope::Child,
        rules: vec![], priority: 0,
        active_from: None,
        active_until: Some(now - chrono::Duration::hours(1)),
        created_at: now, modified_at: now,
    };
    dao::policy::insert(&s, &active).unwrap();
    dao::policy::insert(&s, &expired).unwrap();
    let only_active = dao::policy::list_active(&s, child_id, &now).unwrap();
    assert_eq!(only_active.len(), 1);
    assert_eq!(only_active[0].id, active.id);
}
