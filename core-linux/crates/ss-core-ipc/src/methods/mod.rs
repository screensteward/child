pub mod auth;
pub mod child;
pub mod extension;
pub mod family;
pub mod policy;
pub mod system;
pub mod usage;

use std::collections::HashMap;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::server::MethodHandler;

/// Build the dispatch table.
///
/// Only registers the methods implemented in Task 16 (`auth.*`, `child.*`,
/// `subscribe`, `unsubscribe`). Tasks 17-18 will add `family.*`,
/// `policy.{create,update,delete,list}`, `extension.{grant,listPending,approve,deny}`,
/// `usage.*`, and `system.*`.
///
/// Takes `state` by value: the function clones it into each handler, and
/// passing by value makes the ownership transfer explicit.
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn registry(state: AppState) -> HashMap<String, Arc<dyn MethodHandler>> {
    let mut m: HashMap<String, Arc<dyn MethodHandler>> = HashMap::new();

    // auth.*
    m.insert("auth.login".into(), Arc::new(auth::Login(state.clone())));
    m.insert("auth.logout".into(), Arc::new(auth::Logout));
    m.insert(
        "auth.changePassword".into(),
        Arc::new(auth::ChangePassword(state.clone())),
    );

    // child.* (no auth required — accessible to child UI)
    m.insert(
        "child.getStatus".into(),
        Arc::new(child::GetStatus(state.clone())),
    );
    m.insert(
        "child.getDailyReport".into(),
        Arc::new(child::GetDailyReport(state.clone())),
    );
    m.insert(
        "policy.listActive".into(),
        Arc::new(child::ListActivePolicies(state.clone())),
    );
    m.insert(
        "extension.request".into(),
        Arc::new(child::RequestExtension(state.clone())),
    );

    // subscribe / unsubscribe (no auth, ties connection to fan-out)
    m.insert("subscribe".into(), Arc::new(child::Subscribe));
    m.insert("unsubscribe".into(), Arc::new(child::Unsubscribe));

    m
}
