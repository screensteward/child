use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::{IpcError, Result};
use crate::server::{MethodContext, MethodHandler};
use ss_core_store::dao;

// ── usage.getReport ──────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct GetReport(pub AppState);

#[derive(Deserialize)]
struct ReportParams {
    #[serde(default)]
    child_id: Option<Uuid>,
    #[serde(default)]
    from: Option<NaiveDate>,
    #[serde(default)]
    to: Option<NaiveDate>,
}

#[async_trait]
impl MethodHandler for GetReport {
    fn name(&self) -> &'static str {
        "usage.getReport"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, params: Value) -> Result<Value> {
        let p: ReportParams = serde_json::from_value(params).unwrap_or(ReportParams {
            child_id: None,
            from: None,
            to: None,
        });

        let child_id = if let Some(id) = p.child_id {
            id
        } else {
            let fam = dao::family::get_single(&self.0.store)?
                .ok_or_else(|| IpcError::Rpc("no family".into()))?;
            dao::child::list_by_family(&self.0.store, fam.id)?
                .first()
                .map(|c| c.id)
                .ok_or_else(|| IpcError::Rpc("no child".into()))?
        };

        let to = p.to.unwrap_or_else(|| Utc::now().date_naive());
        let from = p
            .from
            .unwrap_or_else(|| to - chrono::Duration::days(7));

        let mut days = Vec::new();
        let mut d = from;
        while d <= to {
            let m = dao::usage::minutes_for_day(&self.0.store, child_id, d)?;
            days.push(json!({ "date": d, "minutes": m }));
            d = d.succ_opt().unwrap();
        }

        Ok(json!({ "child_id": child_id, "from": from, "to": to, "days": days }))
    }
}
