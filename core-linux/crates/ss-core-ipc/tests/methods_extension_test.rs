use chrono::Utc;
use serde_json::json;
use ss_core_ipc::app_state::AppState;
use ss_core_ipc::auth::ConnState;
use ss_core_ipc::methods::extension::{Approve, Deny, Grant, ListPending};
use ss_core_ipc::methods::family::Bootstrap;
use ss_core_ipc::server::{MethodContext, MethodHandler, NotificationEmitter};
use ss_core_store::dao::exception;
use ss_core_store::Store;
use tempfile::tempdir;

fn setup() -> AppState {
    let d = tempdir().unwrap();
    let s = Store::open_with_key(&d.path().join("c.db"), &[0u8; 32]).unwrap();
    std::mem::forget(d); // keep tempdir alive
    AppState::new(s, NotificationEmitter::new(), false, "0.1.0")
}

/// Returns `(child_id, parent_id)` after bootstrapping the family.
async fn bootstrap(state: &AppState) -> (uuid::Uuid, uuid::Uuid) {
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
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
    let child_id = uuid::Uuid::parse_str(v["child_id"].as_str().unwrap()).unwrap();
    let parent_id = uuid::Uuid::parse_str(v["parent_id"].as_str().unwrap()).unwrap();
    (child_id, parent_id)
}

#[tokio::test]
async fn grant_creates_approved_not_pending() {
    let state = setup();
    let (child_id, parent_id) = bootstrap(&state).await;

    let cs = ConnState::new();
    cs.authenticate(parent_id); // use real parent_id (FK constraint)
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let v = Grant(state.clone())
        .call(
            ctx,
            json!({ "child_id": child_id, "duration_minutes": 15 }),
        )
        .await
        .unwrap();
    assert!(v["id"].is_string(), "grant should return an id");
    assert!(v["expires_at"].is_string(), "grant should return expires_at");

    // A granted exception is already Approved → pending list must be empty.
    let cs2 = ConnState::new();
    cs2.authenticate(parent_id);
    let emitter2 = state.emitter.clone();
    let ctx2 = MethodContext {
        conn_state: &cs2,
        notifier: &emitter2,
    };
    let pending = ListPending(state)
        .call(ctx2, json!({}))
        .await
        .unwrap();
    assert_eq!(
        pending["pending"].as_array().unwrap().len(),
        0,
        "grant creates Approved, so pending list should be empty"
    );
}

#[tokio::test]
async fn grant_emits_extension_granted_notification() {
    let state = setup();
    let (child_id, parent_id) = bootstrap(&state).await;

    let mut rx = state.emitter.subscribe();

    let cs = ConnState::new();
    cs.authenticate(parent_id);
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    Grant(state)
        .call(
            ctx,
            json!({ "child_id": child_id, "duration_minutes": 10 }),
        )
        .await
        .unwrap();

    let (topic, _) = rx.recv().await.unwrap();
    assert_eq!(topic, "extensionGranted");
}

#[tokio::test]
async fn approve_and_deny_emit_notifications() {
    let state = setup();
    let (child_id, parent_id) = bootstrap(&state).await;

    // Create a pending request by inserting directly (simulates child-side request).
    let ticket_id = uuid::Uuid::new_v4();
    exception::insert(
        &state.store,
        &exception::PolicyException {
            id: ticket_id,
            child_id,
            granted_by_parent_id: None,
            status: exception::ExceptionStatus::Pending,
            reason: None,
            duration_minutes: None,
            granted_at: None,
            expires_at: None,
            created_at: Utc::now(),
        },
    )
    .unwrap();

    // Approve it and check notification.
    let mut rx = state.emitter.subscribe();
    let cs = ConnState::new();
    cs.authenticate(parent_id);
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    Approve(state.clone())
        .call(ctx, json!({ "ticket_id": ticket_id }))
        .await
        .unwrap();
    let (topic, _) = rx.recv().await.unwrap();
    assert_eq!(topic, "extensionApproved");

    // Create another pending ticket and deny it.
    let ticket2 = uuid::Uuid::new_v4();
    exception::insert(
        &state.store,
        &exception::PolicyException {
            id: ticket2,
            child_id,
            granted_by_parent_id: None,
            status: exception::ExceptionStatus::Pending,
            reason: None,
            duration_minutes: None,
            granted_at: None,
            expires_at: None,
            created_at: Utc::now(),
        },
    )
    .unwrap();

    let cs3 = ConnState::new();
    cs3.authenticate(parent_id);
    let emitter3 = state.emitter.clone();
    let ctx3 = MethodContext {
        conn_state: &cs3,
        notifier: &emitter3,
    };
    Deny(state)
        .call(ctx3, json!({ "ticket_id": ticket2 }))
        .await
        .unwrap();
    let (topic2, _) = rx.recv().await.unwrap();
    assert_eq!(topic2, "extensionDenied");
}
