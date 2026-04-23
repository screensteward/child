use parking_lot::Mutex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::errors::{EnforceError, Result};

/// Cache key derived from filesystem metadata.
///
/// Two files with the same `(inode, mtime, size)` are assumed to have the same
/// content — avoids re-hashing large ELF binaries on every tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashKey {
    pub inode: u64,
    pub mtime: i64,
    pub size: u64,
}

/// Computes the SHA-256 hash of a file by streaming it in 64 KiB chunks.
///
/// # Errors
///
/// Returns [`EnforceError::Io`] if the file cannot be opened or read.
pub fn compute_content_hash(path: &Path) -> Result<String> {
    let f = File::open(path)?;
    let mut r = BufReader::with_capacity(64 * 1024, f);
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024].into_boxed_slice();
    loop {
        let n = r.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("sha256:{}", hex::encode(h.finalize())))
}

/// Thread-safe LRU-less cache mapping `(inode, mtime, size)` → `sha256:<hex>`.
///
/// Shared across ticks via `Arc`; cheaply cloned.
#[derive(Debug, Clone, Default)]
pub struct ContentHashCache {
    inner: Arc<Mutex<HashMap<HashKey, String>>>,
}

impl ContentHashCache {
    /// Creates a new, empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the cached hash if `(inode, mtime, size)` is known,
    /// otherwise computes it, inserts it, and returns it.
    ///
    /// # Errors
    ///
    /// Returns an error if the file metadata or content cannot be read.
    pub fn get_or_compute(&self, path: &Path) -> Result<String> {
        let md = std::fs::metadata(path)?;
        let key = HashKey {
            inode: md.ino(),
            mtime: md.mtime(),
            size: md.size(),
        };
        if let Some(h) = self.inner.lock().get(&key) {
            return Ok(h.clone());
        }
        let h = compute_content_hash(path)?;
        self.inner.lock().insert(key, h.clone());
        Ok(h)
    }

    /// Returns the number of entries currently in the cache.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.lock().len()
    }

    /// Returns `true` if the cache contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.lock().is_empty()
    }
}

/// A process observed in `/proc` with the attributes needed for policy evaluation.
#[derive(Debug, Clone)]
pub struct ObservedProcess {
    pub pid: i32,
    pub exe_path: PathBuf,
    pub basename: String,
    pub content_hash: String,
    /// Sum of user-mode and kernel-mode CPU jiffies consumed so far.
    pub cpu_jiffies: u64,
}

/// Scans `/proc` and returns all readable processes.
///
/// Entries that disappear mid-scan or are not readable (kernel threads,
/// permission errors) are silently skipped — this is normal `/proc` behaviour.
///
/// # Errors
///
/// Returns [`EnforceError::Cgroup`] if `procfs::process::all_processes()` itself
/// fails (e.g. `/proc` is not mounted).
pub fn scan_processes(cache: &ContentHashCache) -> Result<Vec<ObservedProcess>> {
    use procfs::process::all_processes;

    let mut out = Vec::new();
    for maybe_proc in all_processes().map_err(|e| EnforceError::Cgroup(e.to_string()))? {
        let Ok(p) = maybe_proc else {
            continue;
        };
        let Ok(exe) = p.exe() else {
            continue;
        };
        let Ok(stat) = p.stat() else {
            continue;
        };
        let Ok(hash) = cache.get_or_compute(&exe) else {
            continue;
        };
        let basename = exe
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        out.push(ObservedProcess {
            pid: p.pid,
            exe_path: exe,
            basename,
            content_hash: hash,
            // utime and stime are both u64 in procfs 0.16
            cpu_jiffies: stat.utime + stat.stime,
        });
    }
    Ok(out)
}
