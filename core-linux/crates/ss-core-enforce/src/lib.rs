//! Linux enforcement engine: cgroups v2 freezer, /proc tracker,
//! `DBus` idle detection, tick orchestration.

pub mod cgroup;
pub mod cgroup_mock;
pub mod enforcer;
pub mod errors;
pub mod idle;
pub mod matcher;
pub mod notifier;
pub mod proc_scan;
pub mod tracker;

pub use errors::{EnforceError, Result};
