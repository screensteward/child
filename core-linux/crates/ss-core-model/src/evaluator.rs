use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::policy::{Policy, Rule};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessCandidate {
    pub content_hash: Option<String>,
    pub basename: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Action {
    Allow,
    Warn {
        reason: String,
        remaining_minutes: u32,
    },
    Block {
        reason: String,
    },
}

/// Seuil de warning (en minutes restantes) pour `DailyBudget`.
const WARN_THRESHOLD_MIN: u32 = 5;

/// Évalue l'action à appliquer à un process donné, à un instant donné,
/// sur un stock de minutes consommées (toutes apps du child, aujourd'hui).
///
/// Règle de composition (§6.2 spec fondateur) : si *n'importe quelle*
/// règle active bloque, on bloque. `AppBlocklist` gagne toujours sur
/// `AppAllowlist`. Sinon si une règle `AppAllowlist` est présente et ne
/// matche pas le process, on bloque.
#[must_use]
pub fn evaluate(
    policies: &[Policy],
    process: &ProcessCandidate,
    usage_today_minutes: u32,
    now: &DateTime<Utc>,
) -> Action {
    let mut warn: Option<Action> = None;

    let mut has_allowlist = false;
    let mut allow_match = false;

    for policy in policies {
        if !is_active(policy, now) {
            continue;
        }
        for rule in &policy.rules {
            match rule {
                Rule::AppBlocklist { matchers } => {
                    if matchers.iter().any(|m| {
                        m.matches(
                            process.content_hash.as_deref(),
                            &process.basename,
                            &process.path,
                        )
                    }) {
                        return Action::Block {
                            reason: "blocklist".into(),
                        };
                    }
                }
                Rule::AppAllowlist { matchers } => {
                    has_allowlist = true;
                    if matchers.iter().any(|m| {
                        m.matches(
                            process.content_hash.as_deref(),
                            &process.basename,
                            &process.path,
                        )
                    }) {
                        allow_match = true;
                    }
                }
                Rule::DailyBudget(b) => {
                    if usage_today_minutes >= b.minutes {
                        return Action::Block {
                            reason: "budget_exceeded".into(),
                        };
                    }
                    let remaining = b.minutes - usage_today_minutes;
                    if remaining <= WARN_THRESHOLD_MIN {
                        warn = Some(Action::Warn {
                            reason: "budget_near".into(),
                            remaining_minutes: remaining,
                        });
                    }
                }
                Rule::TimeWindow(tw) => {
                    if !tw.is_open(now) {
                        return Action::Block {
                            reason: "window_closed".into(),
                        };
                    }
                }
            }
        }
    }

    if has_allowlist && !allow_match {
        return Action::Block {
            reason: "not_in_allowlist".into(),
        };
    }

    warn.unwrap_or(Action::Allow)
}

fn is_active(policy: &Policy, now: &DateTime<Utc>) -> bool {
    if let Some(from) = policy.active_from {
        if *now < from {
            return false;
        }
    }
    if let Some(until) = policy.active_until {
        if *now >= until {
            return false;
        }
    }
    true
}
