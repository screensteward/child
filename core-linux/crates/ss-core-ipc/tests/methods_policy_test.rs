use serde_json::json;
use ss_core_ipc::app_state::AppState;
use ss_core_ipc::auth::ConnState;
use ss_core_ipc::methods::family::Bootstrap;
use ss_core_ipc::methods::policy::{Create, List};
use ss_core_ipc::server::{MethodContext, MethodHandler, NotificationEmitter};
use ss_core_store::Store;
use tempfile::tempdir;

fn setup() -> AppState {
    let d = tempdir().unwrap();
    let s = Store::open_with_key(&d.path().join("c.db"), &[0u8; 32]).unwrap();
    std::mem::forget(d); // keep tempdir alive
    AppState::new(s, NotificationEmitter::new(), false, "0.1.0")
}

async fn bootstrap(state: &AppState) -> uuid::Uuid {
    let cs = ConnState::new();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &state.emitter,
    };
    let v = Bootstrap(state.clone())
        .call(
            ctx,
            json!({
                "family_name": "F",
                "parent_display_name": "A",
                "parent_password": "x",
                "child_display_name": "B",
                "hostname": "h"
            }),
        )
        .await
        .unwrap();
    uuid::Uuid::parse_str(v["child_id"].as_str().unwrap()).unwrap()
}

#[tokio::test]
async fn policy_create_then_list() {
    let state = setup();
    let child_id = bootstrap(&state).await;

    // Create a policy.
    let cs = ConnState::new();
    cs.authenticate(uuid::Uuid::new_v4());
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &state.emitter,
    };
    let v = Create(state.clone())
        .call(
            ctx,
            json!({
                "child_id": child_id,
                "scope": "child",
                "rules": [{ "type": "daily_budget", "minutes": 60 }]
            }),
        )
        .await
        .unwrap();
    assert!(v["id"].is_string(), "create should return an id string");

    // List policies for that child.
    let cs2 = ConnState::new();
    cs2.authenticate(uuid::Uuid::new_v4());
    let emitter2 = state.emitter.clone();
    let ctx2 = MethodContext {
        conn_state: &cs2,
        notifier: &emitter2,
    };
    let list = List(state)
        .call(ctx2, json!({ "child_id": child_id }))
        .await
        .unwrap();
    assert_eq!(
        list["policies"].as_array().unwrap().len(),
        1,
        "should have exactly one policy after create"
    );
}

#[tokio::test]
async fn bootstrap_twice_is_rejected() {
    let state = setup();
    bootstrap(&state).await;

    // A second bootstrap attempt must fail.
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let err = Bootstrap(state)
        .call(
            ctx,
            json!({
                "family_name": "F2",
                "parent_display_name": "B",
                "parent_password": "y",
                "child_display_name": "C",
                "hostname": "h2"
            }),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(err, ss_core_ipc::IpcError::Rpc(_)),
        "second bootstrap should return Rpc error, got: {err}"
    );
}

#[tokio::test]
async fn bootstrap_authenticates_connection() {
    let state = setup();
    let cs = ConnState::new();
    assert!(!cs.is_authenticated(), "should start unauthenticated");
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    Bootstrap(state)
        .call(
            ctx,
            json!({
                "family_name": "F",
                "parent_display_name": "A",
                "parent_password": "x",
                "child_display_name": "B",
                "hostname": "h"
            }),
        )
        .await
        .unwrap();
    assert!(
        cs.is_authenticated(),
        "connection should be authenticated after bootstrap"
    );
}
