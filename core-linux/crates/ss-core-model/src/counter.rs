use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// G-Counter CRDT (monotone, merge = max par clé). Tautologique en Phase 1
/// (un seul `ChildDevice`), mais la forme est déjà multi-device pour Phase 2+.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GCounter {
    #[serde(flatten)]
    per_device: BTreeMap<Uuid, u64>,
}

impl GCounter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(&mut self, device: Uuid, by: u64) {
        *self.per_device.entry(device).or_insert(0) += by;
    }

    #[must_use]
    pub fn total(&self) -> u64 {
        self.per_device.values().sum()
    }

    pub fn merge(&mut self, other: &Self) {
        for (d, v) in &other.per_device {
            let cur = self.per_device.entry(*d).or_insert(0);
            if *v > *cur {
                *cur = *v;
            }
        }
    }
}
