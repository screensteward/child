use ss_core_enforce::cgroup::{AppScopeId, CgroupBackend};
use ss_core_enforce::cgroup_mock::MockCgroup;
use ss_core_enforce::enforcer::{Enforcer, EnforcerContext};
use ss_core_enforce::proc_scan::ObservedProcess;
use ss_core_model::evaluator::Action;
use std::path::PathBuf;
use std::sync::Arc;

fn proc(pid: i32, hash: &str) -> ObservedProcess {
    ObservedProcess {
        pid,
        exe_path: PathBuf::from(format!("/bin/p{pid}")),
        basename: format!("p{pid}"),
        content_hash: hash.into(),
        cpu_jiffies: 100,
    }
}

fn scope(hash: &str) -> AppScopeId {
    let hex = hash.trim_start_matches("sha256:");
    let short = &hex[..16.min(hex.len())];
    AppScopeId(format!("app-{short}"))
}

#[tokio::test]
async fn enforcer_freezes_on_block_action() {
    let cg = Arc::new(MockCgroup::new());
    let enf = Enforcer::new(cg.clone());
    let p = proc(1234, "sha256:abcdef0123456789");
    let ctx = EnforcerContext {
        process: &p,
        action: Action::Block {
            reason: "blocklist".into(),
        },
    };
    enf.apply(&ctx).await.unwrap();
    let id = scope("sha256:abcdef0123456789");
    assert!(cg.scope_exists(&id));
    assert!(cg.pids_in_scope(&id).contains(&1234));
    assert!(cg.is_frozen(&id));
}

#[tokio::test]
async fn enforcer_unfreezes_on_allow_if_previously_frozen() {
    let cg = Arc::new(MockCgroup::new());
    let enf = Enforcer::new(cg.clone());
    let p = proc(1, "sha256:h");
    // First Block, then Allow: scope must be thawed.
    enf.apply(&EnforcerContext {
        process: &p,
        action: Action::Block {
            reason: "blocklist".into(),
        },
    })
    .await
    .unwrap();
    enf.apply(&EnforcerContext {
        process: &p,
        action: Action::Allow,
    })
    .await
    .unwrap();
    let id = scope("sha256:h");
    assert!(!cg.is_frozen(&id));
}

#[tokio::test]
async fn enforcer_does_not_freeze_on_warn() {
    let cg = Arc::new(MockCgroup::new());
    let enf = Enforcer::new(cg.clone());
    let p = proc(1, "sha256:h");
    enf.apply(&EnforcerContext {
        process: &p,
        action: Action::Warn {
            reason: "budget_near".into(),
            remaining_minutes: 3,
        },
    })
    .await
    .unwrap();
    let id = scope("sha256:h");
    assert!(!cg.is_frozen(&id));
}
