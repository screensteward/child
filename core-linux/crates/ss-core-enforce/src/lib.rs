//! Linux enforcement engine: cgroups v2 freezer, /proc tracker,
//! `DBus` idle detection, tick orchestration.

pub mod cgroup;
pub mod cgroup_mock;
pub mod errors;
pub mod matcher;
pub mod proc_scan;

pub use errors::{EnforceError, Result};
