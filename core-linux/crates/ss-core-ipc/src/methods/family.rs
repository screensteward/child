use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::dto::{ChildView, DeviceView, FamilySnapshot, ParentView};
use crate::errors::{IpcError, Result};
use crate::server::{MethodContext, MethodHandler};
use ss_core_model::entities::{Child, ChildDevice, Family, Parent, Platform};
use ss_core_store::dao;

// ── family.get ──────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Get(pub AppState);

#[async_trait]
impl MethodHandler for Get {
    fn name(&self) -> &'static str {
        "family.get"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        let Some(fam) = dao::family::get_single(&self.0.store)? else {
            return Ok(json!({ "family": null }));
        };
        let parents = vec![dao::parent::get_single(&self.0.store)?]
            .into_iter()
            .flatten()
            .map(|p| ParentView {
                id: p.id,
                display_name: p.display_name,
            })
            .collect::<Vec<_>>();
        let children = dao::child::list_by_family(&self.0.store, fam.id)?
            .into_iter()
            .map(|c| ChildView {
                id: c.id,
                display_name: c.display_name,
                birth_year: c.birth_year,
            })
            .collect::<Vec<_>>();
        let devices: Vec<DeviceView> = {
            let mut all = Vec::new();
            for c in &children {
                for d in dao::device::list_by_child(&self.0.store, c.id)? {
                    all.push(DeviceView {
                        id: d.id,
                        child_id: d.child_id,
                        hostname: d.hostname,
                        platform: format!("{:?}", d.platform).to_lowercase(),
                        last_seen_at: d.last_seen_at,
                    });
                }
            }
            all
        };
        let snap = FamilySnapshot {
            family_id: fam.id,
            family_name: fam.name,
            parents,
            children,
            devices,
        };
        Ok(serde_json::to_value(snap)?)
    }
}

// ── child.create ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ChildCreate(pub AppState);

#[derive(Deserialize)]
struct ChildCreateParams {
    display_name: String,
    #[serde(default)]
    birth_year: Option<u16>,
}

#[async_trait]
impl MethodHandler for ChildCreate {
    fn name(&self) -> &'static str {
        "child.create"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: ChildCreateParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let fam = dao::family::get_single(&self.0.store)?
            .ok_or_else(|| IpcError::Rpc("no family — run onboarding first".into()))?;
        let now = Utc::now();
        let c = Child {
            id: Uuid::new_v4(),
            family_id: fam.id,
            display_name: p.display_name,
            birth_year: p.birth_year,
            created_at: now,
            modified_at: now,
        };
        dao::child::insert(&self.0.store, &c)?;
        Ok(json!({ "id": c.id }))
    }
}

// ── child.update ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ChildUpdate(pub AppState);

#[derive(Deserialize)]
struct ChildUpdateParams {
    id: Uuid,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    birth_year: Option<u16>,
}

#[async_trait]
impl MethodHandler for ChildUpdate {
    fn name(&self) -> &'static str {
        "child.update"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: ChildUpdateParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let now = Utc::now();
        self.0.store.with_conn(|c| {
            c.execute(
                "UPDATE child SET
                   display_name = COALESCE(?2, display_name),
                   birth_year   = COALESCE(?3, birth_year),
                   modified_at  = ?4
                 WHERE id = ?1",
                rusqlite::params![
                    p.id.to_string(),
                    p.display_name,
                    p.birth_year.map(i64::from),
                    now.to_rfc3339(),
                ],
            )
            .map_err(ss_core_store::StoreError::from)?;
            Ok::<_, ss_core_store::StoreError>(())
        })?;
        Ok(json!({ "ok": true }))
    }
}

// ── family.bootstrap ────────────────────────────────────────────────────────

/// No-auth one-shot onboarding: creates Family + Parent + Child + `ChildDevice`
/// atomically. Rejected if a parent already exists (Phase 1 = single family).
/// On success, authenticates the current connection so no separate login
/// is needed right after onboarding.
#[derive(Debug)]
pub struct Bootstrap(pub AppState);

#[derive(Deserialize)]
struct BootstrapParams {
    family_name: String,
    parent_display_name: String,
    parent_password: String,
    child_display_name: String,
    #[serde(default)]
    child_birth_year: Option<u16>,
    hostname: String,
}

#[async_trait]
impl MethodHandler for Bootstrap {
    fn name(&self) -> &'static str {
        "family.bootstrap"
    }

    fn requires_auth(&self) -> bool {
        false
    }

    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        if dao::parent::get_single(&self.0.store)?.is_some() {
            return Err(IpcError::Rpc("already bootstrapped".into()));
        }
        let p: BootstrapParams =
            serde_json::from_value(params).map_err(|e| IpcError::InvalidParams(e.to_string()))?;
        let now = Utc::now();
        let fam = Family {
            id: Uuid::new_v4(),
            name: p.family_name,
            created_at: now,
            modified_at: now,
        };
        let hash = crate::auth::hash_password(&p.parent_password)?;
        let parent = Parent {
            id: Uuid::new_v4(),
            family_id: fam.id,
            display_name: p.parent_display_name,
            auth_hash: hash,
            created_at: now,
            modified_at: now,
        };
        let child = Child {
            id: Uuid::new_v4(),
            family_id: fam.id,
            display_name: p.child_display_name,
            birth_year: p.child_birth_year,
            created_at: now,
            modified_at: now,
        };
        let device = ChildDevice {
            id: Uuid::new_v4(),
            child_id: child.id,
            hostname: p.hostname,
            platform: Platform::Linux,
            last_seen_at: now,
        };
        dao::family::insert(&self.0.store, &fam)?;
        dao::parent::insert(&self.0.store, &parent)?;
        dao::child::insert(&self.0.store, &child)?;
        dao::device::insert(&self.0.store, &device)?;
        ctx.conn_state.authenticate(parent.id);
        Ok(json!({
            "family_id": fam.id,
            "parent_id": parent.id,
            "child_id": child.id,
            "device_id": device.id,
        }))
    }
}
