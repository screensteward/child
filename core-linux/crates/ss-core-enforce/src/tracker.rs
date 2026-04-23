//! Tick-based usage tracker with CPU-delta filtering and per-app dedup.
//!
//! The tracker runs once per tick (5 s in production). It inspects the list of
//! currently observed processes, compares each PID's CPU jiffies to its
//! previous sample, and charges `tick_seconds` to every application
//! (`content_hash`) whose delta exceeds a configured threshold. Processes
//! sharing the same `content_hash` are counted once per tick (cap
//! `tick_seconds`/app/tick). When the session is idle no time is counted, but
//! the `prev` map is still refreshed so the next active tick starts from a
//! fresh baseline (no spurious spike at idle exit).

use std::collections::{HashMap, HashSet};

use crate::proc_scan::ObservedProcess;

/// Previous CPU sample for a PID, kept across ticks to compute the delta.
#[derive(Debug, Clone, Copy)]
pub struct PrevCpu {
    /// Cumulative user + kernel jiffies at last observation.
    pub jiffies: u64,
    /// Tick number at which this PID was last observed (used by [`Tracker::gc_prev`]).
    pub last_seen: u64,
}

/// Result of a single [`Tracker::tick`].
///
/// `seconds_by_hash` is the usage (in seconds) to credit per `content_hash`
/// for this tick. `newly_seen_hashes` contains the hashes observed for the
/// first time in the tracker's lifetime — the caller is expected to persist
/// them (e.g. via `ss-core-store::dao::signature::upsert_observation`) and
/// optionally raise `onEnforcementAction(unknown_binary)`.
#[derive(Debug, Clone)]
pub struct TickOutcome {
    /// Seconds credited to each active application (by `content_hash`).
    pub seconds_by_hash: Vec<(String, u32)>,
    /// Hashes observed for the first time during this tick.
    pub newly_seen_hashes: Vec<String>,
}

/// Stateful tracker advancing one tick at a time.
///
/// The tracker itself is mutated only in [`Tracker::finalize_tick`];
/// [`Tracker::tick`] is a pure-ish projection over the current state
/// (it mutates the caller-owned `prev` map but not the tracker).
#[derive(Debug, Clone)]
pub struct Tracker {
    tick_seconds: u32,
    cpu_delta_threshold_jiffies: u64,
    current_tick: u64,
    ever_seen: HashSet<String>,
}

impl Tracker {
    /// Creates a new tracker.
    ///
    /// `tick_seconds` is the wall-clock duration credited to every active
    /// app each tick (5 s in production). `cpu_delta_threshold_jiffies` is
    /// the minimum jiffy delta for a process to be considered "active"
    /// during a tick (filters out dormant apps per spec §7.1).
    #[must_use]
    pub fn new(tick_seconds: u32, cpu_delta_threshold_jiffies: u64) -> Self {
        Self {
            tick_seconds,
            cpu_delta_threshold_jiffies,
            current_tick: 0,
            ever_seen: HashSet::new(),
        }
    }

    /// Runs one tick over `processes`, updating `prev` in place and returning
    /// the resulting [`TickOutcome`].
    ///
    /// The tracker's internal state (tick counter, ever-seen set) is *not*
    /// mutated here — callers should invoke [`Tracker::finalize_tick`] once
    /// they have committed the outcome (e.g. after persisting usage).
    ///
    /// When `session_idle` is true, no time is counted but `prev` is still
    /// refreshed with the current jiffies so the next active tick starts
    /// from a clean baseline (avoids a spike after idle exit).
    #[must_use]
    pub fn tick(
        &self,
        processes: &[ObservedProcess],
        prev: &mut HashMap<i32, PrevCpu>,
        session_idle: bool,
    ) -> TickOutcome {
        let next_tick = self.current_tick.saturating_add(1);

        if session_idle {
            // Still refresh jiffies to avoid a giant delta at idle exit.
            for p in processes {
                prev.insert(
                    p.pid,
                    PrevCpu {
                        jiffies: p.cpu_jiffies,
                        last_seen: next_tick,
                    },
                );
            }
            return TickOutcome {
                seconds_by_hash: vec![],
                newly_seen_hashes: vec![],
            };
        }

        let mut counted: HashSet<String> = HashSet::new();
        let mut seconds_by_hash: Vec<(String, u32)> = Vec::new();

        for p in processes {
            let prev_cpu = prev.get(&p.pid).copied();
            // New process (no prior sample) -> delta unknown -> do not count.
            let delta = prev_cpu.map_or(0, |pc| p.cpu_jiffies.saturating_sub(pc.jiffies));
            let is_active = delta > self.cpu_delta_threshold_jiffies;

            if is_active && !counted.contains(&p.content_hash) {
                counted.insert(p.content_hash.clone());
                seconds_by_hash.push((p.content_hash.clone(), self.tick_seconds));
            }

            prev.insert(
                p.pid,
                PrevCpu {
                    jiffies: p.cpu_jiffies,
                    last_seen: next_tick,
                },
            );
        }

        // Detect brand-new hashes so the caller can persist them. We do NOT
        // update `ever_seen` here — that happens in `finalize_tick`, so the
        // same `TickOutcome` remains idempotent if observed repeatedly.
        let mut newly_seen: Vec<String> = Vec::new();
        let mut newly_seen_set: HashSet<&str> = HashSet::new();
        for p in processes {
            if !self.ever_seen.contains(&p.content_hash)
                && newly_seen_set.insert(p.content_hash.as_str())
            {
                newly_seen.push(p.content_hash.clone());
            }
        }

        TickOutcome {
            seconds_by_hash,
            newly_seen_hashes: newly_seen,
        }
    }

    /// Commits an outcome: advances the internal tick counter and records
    /// the newly-seen hashes so subsequent ticks won't re-flag them.
    pub fn finalize_tick(&mut self, outcome: &TickOutcome) {
        self.current_tick = self.current_tick.saturating_add(1);
        for h in &outcome.newly_seen_hashes {
            self.ever_seen.insert(h.clone());
        }
    }

    /// Purges `prev` entries whose PID has not been observed for more than
    /// `max_ticks_missing` ticks — bounds memory usage against PID churn.
    pub fn gc_prev(&self, prev: &mut HashMap<i32, PrevCpu>, max_ticks_missing: u64) {
        let cutoff = self.current_tick.saturating_sub(max_ticks_missing);
        prev.retain(|_, v| v.last_seen >= cutoff);
    }
}
