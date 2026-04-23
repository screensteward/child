use chrono::{NaiveDate, NaiveTime};
use ss_core_model::policy::{AppMatcher, DailyBudget, Policy, Rule, Scope, TimeWindow};
use uuid::Uuid;

#[test]
fn app_matcher_content_hash_only() {
    let m = AppMatcher {
        content_hash: Some("sha256:abc".into()),
        basename: None,
        path_glob: None,
    };
    assert!(m.matches(Some("sha256:abc"), "anything", "/any/path"));
    assert!(!m.matches(Some("sha256:other"), "anything", "/any/path"));
    assert!(!m.matches(None, "anything", "/any/path"));
}

#[test]
fn app_matcher_basename_case_insensitive() {
    let m = AppMatcher {
        content_hash: None,
        basename: Some("Steam".into()),
        path_glob: None,
    };
    assert!(m.matches(None, "steam", "/usr/bin/steam"));
    assert!(m.matches(None, "STEAM", "/home/user/bin/STEAM"));
    assert!(!m.matches(None, "firefox", "/usr/bin/firefox"));
}

#[test]
fn app_matcher_path_glob() {
    let m = AppMatcher {
        content_hash: None,
        basename: None,
        path_glob: Some("/home/**/Minecraft*".into()),
    };
    assert!(m.matches(None, "launcher", "/home/enfant/games/MinecraftLauncher"));
    assert!(!m.matches(None, "launcher", "/usr/bin/MinecraftLauncher"));
}

#[test]
fn app_matcher_union_of_fields() {
    // hash mismatch mais basename match -> match global
    let m = AppMatcher {
        content_hash: Some("sha256:expected".into()),
        basename: Some("steam".into()),
        path_glob: None,
    };
    assert!(m.matches(Some("sha256:different"), "steam", "/anywhere"));
}

#[test]
fn daily_budget_parses_json() {
    let j = r#"{"minutes": 120}"#;
    let b: DailyBudget = serde_json::from_str(j).unwrap();
    assert_eq!(b.minutes, 120);
}

#[test]
fn time_window_open_at() {
    let tw = TimeWindow {
        days: vec![1, 2, 3, 4, 5], // lun-ven
        start: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        end: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
    };
    let wed_17h = NaiveDate::from_ymd_opt(2026, 4, 22).unwrap() // mercredi
        .and_hms_opt(17, 0, 0).unwrap();
    assert!(tw.is_open(&wed_17h.and_utc()));
    let sat_17h = NaiveDate::from_ymd_opt(2026, 4, 25).unwrap() // samedi
        .and_hms_opt(17, 0, 0).unwrap();
    assert!(!tw.is_open(&sat_17h.and_utc()));
}

#[test]
fn policy_rule_json_discriminated() {
    let r = Rule::DailyBudget(DailyBudget { minutes: 60 });
    let j = serde_json::to_string(&r).unwrap();
    assert!(j.contains("\"type\":\"daily_budget\""));
    assert!(j.contains("\"minutes\":60"));
    let back: Rule = serde_json::from_str(&j).unwrap();
    assert_eq!(r, back);
}

#[test]
fn scope_child_serializes() {
    let s = Scope::Child;
    let j = serde_json::to_string(&s).unwrap();
    assert_eq!(j, "\"child\"");
}

#[test]
fn policy_has_deterministic_priority_default() {
    let p = Policy {
        id: Uuid::new_v4(),
        child_id: Uuid::new_v4(),
        scope: Scope::Child,
        rules: vec![Rule::DailyBudget(DailyBudget { minutes: 120 })],
        priority: 0,
        active_from: None,
        active_until: None,
        created_at: chrono::Utc::now(),
        modified_at: chrono::Utc::now(),
    };
    assert_eq!(p.priority, 0);
}
