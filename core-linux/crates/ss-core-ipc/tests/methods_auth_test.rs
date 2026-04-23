use chrono::Utc;
use serde_json::json;
use ss_core_ipc::app_state::AppState;
use ss_core_ipc::auth::{hash_password, ConnState};
use ss_core_ipc::methods::auth::{Login, Logout};
use ss_core_ipc::server::{MethodContext, MethodHandler, NotificationEmitter};
use ss_core_model::entities::{Family, Parent};
use ss_core_store::{dao, Store};
use tempfile::tempdir;
use uuid::Uuid;

fn setup() -> AppState {
    let d = tempdir().unwrap();
    let s = Store::open_with_key(&d.path().join("c.db"), &[0u8; 32]).unwrap();
    std::mem::forget(d); // keep the tempdir alive for the duration of the test
    let fam_id = Uuid::new_v4();
    dao::family::insert(
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
        display_name: "A".into(),
        auth_hash: hash_password("hunter2").unwrap(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    dao::parent::insert(&s, &p).unwrap();
    AppState::new(s, NotificationEmitter::new(), false, "0.1.0")
}

#[tokio::test]
async fn login_success_marks_conn_authenticated() {
    let state = setup();
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let h = Login(state);
    let v = h.call(ctx, json!({ "password": "hunter2" })).await.unwrap();
    assert_eq!(v["ok"], json!(true));
    assert!(cs.is_authenticated());
}

#[tokio::test]
async fn login_failure_returns_unauthorized() {
    let state = setup();
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let h = Login(state);
    let err = h
        .call(ctx, json!({ "password": "wrong" }))
        .await
        .unwrap_err();
    assert!(matches!(err, ss_core_ipc::IpcError::Unauthorized));
    assert!(!cs.is_authenticated());
}

#[tokio::test]
async fn logout_clears_auth() {
    let cs = ConnState::new();
    cs.authenticate(Uuid::new_v4());
    let state = setup();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let h = Logout;
    h.call(ctx, json!({})).await.unwrap();
    assert!(!cs.is_authenticated());
}
