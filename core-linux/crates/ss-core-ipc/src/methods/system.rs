use async_trait::async_trait;
use serde_json::Value;

use crate::app_state::AppState;
use crate::dto::CoreStatus;
use crate::errors::Result;
use crate::server::{MethodContext, MethodHandler};

// ── system.getCoreStatus ─────────────────────────────────────────────────────

#[derive(Debug)]
pub struct GetCoreStatus(pub AppState);

#[async_trait]
impl MethodHandler for GetCoreStatus {
    fn name(&self) -> &'static str {
        "system.getCoreStatus"
    }

    fn requires_auth(&self) -> bool {
        true
    }

    async fn call(&self, _ctx: MethodContext<'_>, _params: Value) -> Result<Value> {
        let db_ok = self
            .0
            .store
            .with_conn(|c| {
                c.execute_batch("SELECT 1")
                    .map(|()| true)
                    .map_err(ss_core_store::StoreError::from)
            })
            .unwrap_or(false);

        let status = CoreStatus {
            version: self.0.version.clone(),
            uptime_seconds: self.0.uptime_seconds(),
            tpm_used: self.0.tpm_used,
            db_ok,
            last_enforcement_error: None,
        };

        Ok(serde_json::to_value(status)?)
    }
}
