use chrono::Utc;
use serde_json::json;
use ss_core_ipc::app_state::AppState;
use ss_core_ipc::auth::ConnState;
use ss_core_ipc::methods::child::{GetStatus, RequestExtension, Subscribe};
use ss_core_ipc::server::{MethodContext, MethodHandler, NotificationEmitter};
use ss_core_model::entities::{Child, ChildDevice, Family, Platform};
use ss_core_store::{dao, Store};
use tempfile::tempdir;
use uuid::Uuid;

fn setup() -> (AppState, Uuid) {
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
    let child_id = Uuid::new_v4();
    dao::child::insert(
        &s,
        &Child {
            id: child_id,
            family_id: fam_id,
            display_name: "B".into(),
            birth_year: None,
            created_at: Utc::now(),
            modified_at: Utc::now(),
        },
    )
    .unwrap();
    dao::device::insert(
        &s,
        &ChildDevice {
            id: Uuid::new_v4(),
            child_id,
            hostname: "h".into(),
            platform: Platform::Linux,
            last_seen_at: Utc::now(),
        },
    )
    .unwrap();
    (AppState::new(s, NotificationEmitter::new(), false, "0.1.0"), child_id)
}

#[tokio::test]
async fn get_status_returns_structure() {
    let (state, _) = setup();
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let v = GetStatus(state).call(ctx, json!({})).await.unwrap();
    // No active policy → budget None, window open = true.
    assert!(v.get("today_minutes_used").is_some());
    assert!(v.get("current_window_open").is_some());
}

#[tokio::test]
async fn request_extension_emits_notification() {
    let (state, _child_id) = setup();
    let mut rx = state.emitter.subscribe();
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    let v = RequestExtension(state)
        .call(ctx, json!({"reason": "devoir"}))
        .await
        .unwrap();
    assert!(v["ticket_id"].is_string());
    let (topic, _params) = rx.recv().await.unwrap();
    assert_eq!(topic, "extensionRequested");
}

#[tokio::test]
async fn subscribe_records_topics() {
    let (state, _) = setup();
    let cs = ConnState::new();
    let emitter = state.emitter.clone();
    let ctx = MethodContext {
        conn_state: &cs,
        notifier: &emitter,
    };
    Subscribe
        .call(
            ctx,
            json!({"topics": ["usageUpdate", "policyChanged"]}),
        )
        .await
        .unwrap();
    assert!(cs.is_subscribed("usageUpdate"));
    assert!(cs.is_subscribed("policyChanged"));
    assert!(!cs.is_subscribed("other"));
}
