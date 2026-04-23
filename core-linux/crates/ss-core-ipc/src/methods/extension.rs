use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::{IpcError, Result};
use crate::server::{MethodContext, MethodHandler};
use ss_core_store::dao::exception;

// ── extension.grant ─────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Grant(pub AppState);

#[derive(Deserialize)]
struct GrantParams {
    child_id: Uuid,
    duration_minutes: u32,
    #[serde(default)]
    reason: Option<String>,
}

#[async_trait]
impl MethodHandler for Grant {
    fn name(&self) -> &'static str {
        "extension.grant"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: GrantParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let parent_id = ctx.conn_state.parent_id().ok_or(IpcError::Unauthorized)?;
        let now = Utc::now();
        let expires = now + Duration::minutes(i64::from(p.duration_minutes));
        let id = Uuid::new_v4();
        exception::insert(
            &self.0.store,
            &exception::PolicyException {
                id,
                child_id: p.child_id,
                granted_by_parent_id: Some(parent_id),
                status: exception::ExceptionStatus::Approved,
                reason: p.reason,
                duration_minutes: Some(p.duration_minutes),
                granted_at: Some(now),
                expires_at: Some(expires),
                created_at: now,
            },
        )?;
        ctx.notifier.emit(
            "extensionGranted",
            json!({
                "id": id,
                "child_id": p.child_id,
                "expires_at": expires,
            }),
        );
        Ok(json!({ "id": id, "expires_at": expires }))
    }
}

// ── extension.listPending ───────────────────────────────────────────────────

#[derive(Debug)]
pub struct ListPending(pub AppState);

#[async_trait]
impl MethodHandler for ListPending {
    fn name(&self) -> &'static str {
        "extension.listPending"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        let rows = exception::list_pending(&self.0.store)?;
        Ok(json!({ "pending": rows }))
    }
}

// ── extension.approve ───────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Approve(pub AppState);

#[derive(Deserialize)]
struct ApproveParams {
    ticket_id: Uuid,
    #[serde(default)]
    duration_minutes: Option<u32>,
}

#[async_trait]
impl MethodHandler for Approve {
    fn name(&self) -> &'static str {
        "extension.approve"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: ApproveParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let parent_id = ctx.conn_state.parent_id().ok_or(IpcError::Unauthorized)?;
        let now = Utc::now();
        let duration = p.duration_minutes.unwrap_or(30);
        let expires = now + Duration::minutes(i64::from(duration));
        exception::update_status(
            &self.0.store,
            p.ticket_id,
            &exception::ExceptionStatus::Approved,
            Some(parent_id),
            Some(duration),
            Some(expires),
            Some(now),
        )?;
        ctx.notifier.emit(
            "extensionApproved",
            json!({ "id": p.ticket_id, "expires_at": expires }),
        );
        Ok(json!({ "expires_at": expires }))
    }
}

// ── extension.deny ──────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Deny(pub AppState);

#[derive(Deserialize)]
struct DenyParams {
    ticket_id: Uuid,
}

#[async_trait]
impl MethodHandler for Deny {
    fn name(&self) -> &'static str {
        "extension.deny"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: DenyParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let parent_id = ctx.conn_state.parent_id().ok_or(IpcError::Unauthorized)?;
        exception::update_status(
            &self.0.store,
            p.ticket_id,
            &exception::ExceptionStatus::Denied,
            Some(parent_id),
            None,
            None,
            Some(Utc::now()),
        )?;
        ctx.notifier
            .emit("extensionDenied", json!({ "id": p.ticket_id }));
        Ok(json!({ "ok": true }))
    }
}
