use ss_core_enforce::cgroup::{AppScopeId, CgroupBackend, CgroupV2};
use tempfile::tempdir;

/// Real cgroup v2 test. Run as root or inside a `--privileged` container.
/// Requires `/sys/fs/cgroup` unified-mounted with write permissions.
#[test]
#[ignore = "requires root and cgroup v2 unified"]
fn cgroup_v2_ensure_scope_creates_directory() {
    let tmp = tempdir().unwrap();
    // Note: `/sys/fs/cgroup` is not writable from a tempdir; this test
    // expects an environment where `root/cgroup.procs` is writable. Here we
    // redirect to a tempdir to validate directory-creation semantics only
    // (real cgroup writes are covered by the E2E test in Task 29).
    let bk = CgroupV2::new(tmp.path());
    let id = AppScopeId("app-test".into());
    bk.ensure_scope(&id).unwrap();
    assert!(bk.scope_exists(&id));
}
