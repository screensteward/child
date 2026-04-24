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

    // family.* + child.create/update (parent-facing, authenticated)
    m.insert(
        "family.bootstrap".into(),
        Arc::new(family::Bootstrap(state.clone())),
    );
    m.insert("family.get".into(), Arc::new(family::Get(state.clone())));
    m.insert(
        "child.create".into(),
        Arc::new(family::ChildCreate(state.clone())),
    );
    m.insert(
        "child.update".into(),
        Arc::new(family::ChildUpdate(state.clone())),
    );

    // policy.* (parent-facing, authenticated)
    m.insert(
        "policy.create".into(),
        Arc::new(policy::Create(state.clone())),
    );
    m.insert(
        "policy.update".into(),
        Arc::new(policy::Update(state.clone())),
    );
    m.insert(
        "policy.delete".into(),
        Arc::new(policy::Delete(state.clone())),
    );
    m.insert("policy.list".into(), Arc::new(policy::List(state.clone())));

    // extension.* parent side (authenticated)
    m.insert(
        "extension.grant".into(),
        Arc::new(extension::Grant(state.clone())),
    );
    m.insert(
        "extension.listPending".into(),
        Arc::new(extension::ListPending(state.clone())),
    );
    m.insert(
        "extension.approve".into(),
        Arc::new(extension::Approve(state.clone())),
    );
    m.insert(
        "extension.deny".into(),
        Arc::new(extension::Deny(state.clone())),
    );

    // usage.* (parent-facing, authenticated)
    m.insert(
        "usage.getReport".into(),
        Arc::new(usage::GetReport(state.clone())),
    );

    // system.* (parent-facing, authenticated)
    m.insert(
        "system.getCoreStatus".into(),
        Arc::new(system::GetCoreStatus(state.clone())),
    );

    m
}
