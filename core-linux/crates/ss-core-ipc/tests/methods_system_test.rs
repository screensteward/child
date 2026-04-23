use serde_json::json;
use ss_core_ipc::app_state::AppState;
use ss_core_ipc::auth::ConnState;
use ss_core_ipc::methods::system::GetCoreStatus;
use ss_core_ipc::server::{MethodContext, MethodHandler, NotificationEmitter};
use ss_core_store::Store;
use tempfile::tempdir;
use uuid::Uuid;

#[tokio::test]
async fn core_status_exposes_version_uptime_tpm() {
    let d = tempdir().unwrap();
    let s = Store::open_with_key(&d.path().join("c.db"), &[0u8; 32]).unwrap();
    std::mem::forget(d);
    let state = AppState::new(s, NotificationEmitter::new(), true, "0.1.0");
    let cs = ConnState::new();
    cs.authenticate(Uuid::new_v4());
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &state.emitter,
    };
    let v = GetCoreStatus(state.clone()).call(ctx, json!({})).await.unwrap();
    assert_eq!(v["version"], json!("0.1.0"));
    assert_eq!(v["tpm_used"], json!(true));
    assert_eq!(v["db_ok"], json!(true));
    assert!(v["uptime_seconds"].as_u64().unwrap() < 5);
}
