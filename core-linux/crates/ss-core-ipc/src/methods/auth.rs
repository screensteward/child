//! `auth.*` IPC method handlers.
//!
//! - `auth.login`          — no auth required; authenticates the connection.
//! - `auth.logout`         — requires auth; clears the connection auth flag.
//! - `auth.changePassword` — requires auth; rehashes and persists the parent password.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use ss_core_store::dao::parent;

use crate::app_state::AppState;
use crate::auth::{hash_password, verify_password};
use crate::errors::{IpcError, Result};
use crate::server::{MethodContext, MethodHandler};

// ── Login ──────────────────────────────────────────────────────────────────

/// Authenticates the current IPC connection using the parent password.
#[derive(Debug)]
pub struct Login(pub AppState);

#[derive(Deserialize)]
struct LoginParams {
    password: String,
}

#[async_trait]
impl MethodHandler for Login {
    fn name(&self) -> &'static str {
        "auth.login"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: LoginParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;

        let Some(parent_row) = parent::get_single(&self.0.store)? else {
            return Err(IpcError::Rpc(
                "no parent configured (run onboarding)".into(),
            ));
        };

        if !verify_password(&p.password, &parent_row.auth_hash)? {
            return Err(IpcError::Unauthorized);
        }

        ctx.conn_state.authenticate(parent_row.id);
        Ok(json!({ "ok": true }))
    }
}

// ── Logout ─────────────────────────────────────────────────────────────────

/// Clears the authentication flag on the current IPC connection.
#[derive(Debug)]
pub struct Logout;

#[async_trait]
impl MethodHandler for Logout {
    fn name(&self) -> &'static str {
        "auth.logout"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        ctx.conn_state.logout();
        Ok(json!({ "ok": true }))
    }
}

// ── ChangePassword ─────────────────────────────────────────────────────────

/// Verifies the current password and replaces it with a new argon2id hash.
#[derive(Debug)]
pub struct ChangePassword(pub AppState);

#[derive(Deserialize)]
struct ChangePasswordParams {
    old: String,
    new: String,
}

#[async_trait]
impl MethodHandler for ChangePassword {
    fn name(&self) -> &'static str {
        "auth.changePassword"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: ChangePasswordParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;

        let Some(parent_row) = parent::get_single(&self.0.store)? else {
            return Err(IpcError::Rpc("no parent configured".into()));
        };

        if !verify_password(&p.old, &parent_row.auth_hash)? {
            return Err(IpcError::Unauthorized);
        }

        let new_hash = hash_password(&p.new)?;
        parent::update_password_hash(&self.0.store, parent_row.id, &new_hash)?;

        // Phase 1: single parent connection — no cross-connection invalidation needed.
        Ok(json!({ "ok": true }))
    }
}
