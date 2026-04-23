use ss_core_enforce::cgroup::{AppScopeId, CgroupBackend};
use ss_core_enforce::cgroup_mock::MockCgroup;

#[test]
fn mock_freeze_unfreeze_tracked() {
    let m = MockCgroup::new();
    let id = AppScopeId("abc".into());
    m.ensure_scope(&id).unwrap();
    m.move_pid(&id, 1234).unwrap();
    assert!(m.scope_exists(&id));
    assert!(m.pids_in_scope(&id).contains(&1234));

    m.freeze(&id).unwrap();
    assert!(m.is_frozen(&id));
    m.unfreeze(&id).unwrap();
    assert!(!m.is_frozen(&id));
}
