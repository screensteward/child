use ss_core_enforce::matcher::match_process;
use ss_core_model::policy::{AppMatcher, DailyBudget, Policy, Rule, Scope};
use ss_core_model::evaluator::ProcessCandidate;
use chrono::Utc;
use uuid::Uuid;

fn pol_with(rule: Rule) -> Policy {
    Policy {
        id: Uuid::new_v4(),
        child_id: Uuid::new_v4(),
        scope: Scope::Child,
        rules: vec![rule],
        priority: 0,
        active_from: None,
        active_until: None,
        created_at: Utc::now(),
        modified_at: Utc::now(),
    }
}

#[test]
fn match_finds_any_rule_referencing_process() {
    let pol = pol_with(Rule::AppBlocklist {
        matchers: vec![AppMatcher {
            content_hash: None,
            basename: Some("steam".into()),
            path_glob: None,
        }],
    });
    let p = ProcessCandidate {
        content_hash: Some("sha256:xyz".into()),
        basename: "steam".into(),
        path: "/usr/bin/steam".into(),
    };
    assert!(match_process(&[pol], &p));
}

#[test]
fn match_false_when_no_rule_references_process() {
    let pol = pol_with(Rule::DailyBudget(DailyBudget { minutes: 60 }));
    let p = ProcessCandidate {
        content_hash: Some("sha256:xyz".into()),
        basename: "firefox".into(),
        path: "/usr/bin/firefox".into(),
    };
    // daily_budget is not a process matcher — no match.
    assert!(!match_process(&[pol], &p));
}
