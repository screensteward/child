use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
#[allow(unused_imports)]
use ss_core_model::entities::Platform;
use ss_core_model::evaluator::{evaluate, Action, ProcessCandidate};
use ss_core_model::policy::{AppMatcher, DailyBudget, Policy, Rule, Scope, TimeWindow};
use uuid::Uuid;

fn mk_policy(rules: Vec<Rule>) -> Policy {
    Policy {
        id: Uuid::new_v4(),
        child_id: Uuid::new_v4(),
        scope: Scope::Child,
        rules,
        priority: 0,
        active_from: None,
        active_until: None,
        created_at: Utc::now(),
        modified_at: Utc::now(),
    }
}

fn proc_at(hash: &str, basename: &str, path: &str) -> ProcessCandidate {
    ProcessCandidate {
        content_hash: Some(hash.into()),
        basename: basename.into(),
        path: path.into(),
    }
}

#[test]
fn evaluator_allows_when_no_rule_matches() {
    let pol = mk_policy(vec![Rule::AppBlocklist {
        matchers: vec![AppMatcher {
            content_hash: None,
            basename: Some("steam".into()),
            path_glob: None,
        }],
    }]);
    let p = proc_at("sha256:abc", "firefox", "/usr/bin/firefox");
    let now = Utc::now();
    let usage_minutes = 0;
    assert_eq!(evaluate(&[pol], &p, usage_minutes, &now), Action::Allow);
}

#[test]
fn evaluator_blocks_on_blocklist_match() {
    let pol = mk_policy(vec![Rule::AppBlocklist {
        matchers: vec![AppMatcher {
            content_hash: None,
            basename: Some("steam".into()),
            path_glob: None,
        }],
    }]);
    let p = proc_at("sha256:abc", "steam", "/usr/bin/steam");
    assert_eq!(
        evaluate(&[pol], &p, 0, &Utc::now()),
        Action::Block {
            reason: "blocklist".into()
        }
    );
}

#[test]
fn evaluator_blocks_when_budget_exceeded() {
    let pol = mk_policy(vec![Rule::DailyBudget(DailyBudget { minutes: 60 })]);
    let p = proc_at("sha256:abc", "firefox", "/usr/bin/firefox");
    let now = Utc::now();
    assert_eq!(
        evaluate(std::slice::from_ref(&pol), &p, 60, &now),
        Action::Block {
            reason: "budget_exceeded".into()
        }
    );
    // 50 min used → 10 min remaining, above warn threshold → Allow
    assert_eq!(
        evaluate(std::slice::from_ref(&pol), &p, 50, &now),
        Action::Allow
    );
}

#[test]
fn evaluator_warns_near_budget() {
    let pol = mk_policy(vec![Rule::DailyBudget(DailyBudget { minutes: 60 })]);
    let p = proc_at("sha256:abc", "firefox", "/usr/bin/firefox");
    // 5 min restant → warn
    assert_eq!(
        evaluate(&[pol], &p, 55, &Utc::now()),
        Action::Warn {
            reason: "budget_near".into(),
            remaining_minutes: 5
        }
    );
}

#[test]
fn evaluator_blocks_outside_time_window() {
    let tw = TimeWindow {
        days: vec![1, 2, 3, 4, 5],
        start: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        end: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
    };
    let pol = mk_policy(vec![Rule::TimeWindow(tw)]);
    let p = proc_at("sha256:abc", "firefox", "/usr/bin/firefox");
    let monday_10h: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 4, 20)
        .unwrap()
        .and_hms_opt(10, 0, 0)
        .unwrap()
        .and_utc();
    assert_eq!(
        evaluate(&[pol], &p, 0, &monday_10h),
        Action::Block {
            reason: "window_closed".into()
        }
    );
}

#[test]
fn evaluator_allowlist_blocks_everything_else() {
    let pol = mk_policy(vec![Rule::AppAllowlist {
        matchers: vec![AppMatcher {
            content_hash: None,
            basename: Some("firefox".into()),
            path_glob: None,
        }],
    }]);
    let steam = proc_at("sha256:abc", "steam", "/usr/bin/steam");
    let firefox = proc_at("sha256:def", "firefox", "/usr/bin/firefox");
    assert_eq!(
        evaluate(std::slice::from_ref(&pol), &steam, 0, &Utc::now()),
        Action::Block {
            reason: "not_in_allowlist".into()
        }
    );
    assert_eq!(evaluate(&[pol], &firefox, 0, &Utc::now()), Action::Allow);
}

#[test]
fn evaluator_blocklist_wins_over_allowlist() {
    // Allowlist {firefox}, Blocklist {firefox} → bloqué (restriction plus forte gagne).
    let pol = mk_policy(vec![
        Rule::AppAllowlist {
            matchers: vec![AppMatcher {
                content_hash: None,
                basename: Some("firefox".into()),
                path_glob: None,
            }],
        },
        Rule::AppBlocklist {
            matchers: vec![AppMatcher {
                content_hash: None,
                basename: Some("firefox".into()),
                path_glob: None,
            }],
        },
    ]);
    let firefox = proc_at("sha256:def", "firefox", "/usr/bin/firefox");
    assert_eq!(
        evaluate(&[pol], &firefox, 0, &Utc::now()),
        Action::Block {
            reason: "blocklist".into()
        }
    );
}
