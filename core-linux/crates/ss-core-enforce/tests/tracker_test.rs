use ss_core_enforce::proc_scan::ObservedProcess;
use ss_core_enforce::tracker::{PrevCpu, Tracker};
use std::collections::HashMap;
use std::path::PathBuf;

fn proc(pid: i32, hash: &str, jiffies: u64) -> ObservedProcess {
    ObservedProcess {
        pid,
        exe_path: PathBuf::from(format!("/bin/p{pid}")),
        basename: format!("p{pid}"),
        content_hash: hash.into(),
        cpu_jiffies: jiffies,
    }
}

#[test]
fn tracker_counts_only_active_processes() {
    let t = Tracker::new(5, 10); // tick 5s, cpu_delta_threshold 10 jiffies
    let mut prev: HashMap<i32, PrevCpu> = HashMap::new();
    prev.insert(
        1,
        PrevCpu {
            jiffies: 100,
            last_seen: 0,
        },
    );

    // tick n : pid 1 a consommé 50 jiffies (actif), pid 2 vient d'apparaître (0).
    let processes = vec![proc(1, "h1", 150), proc(2, "h2", 0)];
    let outcome = t.tick(&processes, &mut prev, /* session_idle = */ false);

    // Seul pid 1 a delta > threshold → son app compte 5s.
    let h1_sec: u32 = outcome
        .seconds_by_hash
        .iter()
        .find(|(h, _)| h == "h1")
        .map_or(0, |(_, s)| *s);
    let h2_sec: u32 = outcome
        .seconds_by_hash
        .iter()
        .find(|(h, _)| h == "h2")
        .map_or(0, |(_, s)| *s);
    assert_eq!(h1_sec, 5);
    assert_eq!(h2_sec, 0);
}

#[test]
fn tracker_counts_nothing_when_idle() {
    let t = Tracker::new(5, 10);
    let mut prev: HashMap<i32, PrevCpu> = HashMap::new();
    prev.insert(
        1,
        PrevCpu {
            jiffies: 100,
            last_seen: 0,
        },
    );
    let processes = vec![proc(1, "h1", 500)];
    let out = t.tick(&processes, &mut prev, true);
    assert!(out.seconds_by_hash.is_empty());
}

#[test]
fn tracker_deduplicates_same_hash_across_pids() {
    let t = Tracker::new(5, 10);
    let mut prev = HashMap::new();
    prev.insert(
        1,
        PrevCpu {
            jiffies: 100,
            last_seen: 0,
        },
    );
    prev.insert(
        2,
        PrevCpu {
            jiffies: 100,
            last_seen: 0,
        },
    );
    let processes = vec![proc(1, "h1", 200), proc(2, "h1", 200)];
    let out = t.tick(&processes, &mut prev, false);
    let total: u32 = out
        .seconds_by_hash
        .iter()
        .find(|(h, _)| h == "h1")
        .map_or(0, |(_, s)| *s);
    // 5 s max par tick par app, même si plusieurs pids.
    assert_eq!(total, 5);
}
