use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::cgroup::{AppScopeId, CgroupBackend};
use crate::errors::Result;

#[derive(Debug, Default)]
struct State {
    scopes: HashMap<AppScopeId, ScopeState>,
}

#[derive(Debug, Default)]
struct ScopeState {
    pids: HashSet<i32>,
    frozen: bool,
    killed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MockCgroup {
    inner: Arc<Mutex<State>>,
}

impl MockCgroup {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn pids_in_scope(&self, id: &AppScopeId) -> Vec<i32> {
        self.inner
            .lock()
            .scopes
            .get(id)
            .map(|s| s.pids.iter().copied().collect())
            .unwrap_or_default()
    }

    #[must_use]
    pub fn was_killed(&self, id: &AppScopeId) -> bool {
        self.inner
            .lock()
            .scopes
            .get(id)
            .is_some_and(|s| s.killed)
    }
}

impl CgroupBackend for MockCgroup {
    fn ensure_scope(&self, id: &AppScopeId) -> Result<()> {
        self.inner.lock().scopes.entry(id.clone()).or_default();
        Ok(())
    }

    fn move_pid(&self, id: &AppScopeId, pid: i32) -> Result<()> {
        self.inner
            .lock()
            .scopes
            .entry(id.clone())
            .or_default()
            .pids
            .insert(pid);
        Ok(())
    }

    fn freeze(&self, id: &AppScopeId) -> Result<()> {
        self.inner
            .lock()
            .scopes
            .entry(id.clone())
            .or_default()
            .frozen = true;
        Ok(())
    }

    fn unfreeze(&self, id: &AppScopeId) -> Result<()> {
        self.inner
            .lock()
            .scopes
            .entry(id.clone())
            .or_default()
            .frozen = false;
        Ok(())
    }

    fn kill_scope(&self, id: &AppScopeId) -> Result<()> {
        if let Some(s) = self.inner.lock().scopes.get_mut(id) {
            s.killed = true;
            s.pids.clear();
        }
        Ok(())
    }

    fn scope_exists(&self, id: &AppScopeId) -> bool {
        self.inner.lock().scopes.contains_key(id)
    }

    fn is_frozen(&self, id: &AppScopeId) -> bool {
        self.inner
            .lock()
            .scopes
            .get(id)
            .is_some_and(|s| s.frozen)
    }
}
