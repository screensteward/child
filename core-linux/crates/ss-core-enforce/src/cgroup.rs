use async_trait::async_trait;
use std::fmt;

use crate::errors::Result;

/// Opaque cgroup scope identifier. In Phase 1 = `app-` prefix + first 16 hex
/// characters of `content_hash`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppScopeId(pub String);

impl fmt::Display for AppScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Abstraction over the cgroup backend. Implemented by `CgroupV2` (prod) and
/// `MockCgroup` (tests).
///
/// # Errors
///
/// Each fallible method returns [`EnforceError`](crate::errors::EnforceError)
/// when the underlying backend cannot satisfy the request (I/O failure on the
/// real backend, never on the mock).
pub trait CgroupBackend: Send + Sync {
    /// # Errors
    /// Returns an error if the scope directory cannot be created.
    fn ensure_scope(&self, id: &AppScopeId) -> Result<()>;
    /// # Errors
    /// Returns an error if writing the PID to `cgroup.procs` fails.
    fn move_pid(&self, id: &AppScopeId, pid: i32) -> Result<()>;
    /// # Errors
    /// Returns an error if writing to `cgroup.freeze` fails.
    fn freeze(&self, id: &AppScopeId) -> Result<()>;
    /// # Errors
    /// Returns an error if writing to `cgroup.freeze` fails.
    fn unfreeze(&self, id: &AppScopeId) -> Result<()>;
    /// # Errors
    /// Returns an error if writing to `cgroup.kill` fails.
    fn kill_scope(&self, id: &AppScopeId) -> Result<()>;
    fn scope_exists(&self, id: &AppScopeId) -> bool;
    fn is_frozen(&self, id: &AppScopeId) -> bool;
}

/// Real cgroups v2 implementation — requires `/sys/fs/cgroup` unified-mounted
/// and root privileges (or `cgroup delegation`).
///
/// Layout: `/sys/fs/cgroup/screensteward.slice/app-<short>.scope/`
///   - `cgroup.procs`  — write a PID to move it into the scope
///   - `cgroup.freeze` — write `"1\n"` to freeze, `"0\n"` to thaw
///   - `cgroup.kill`   — write `"1\n"` to SIGKILL the whole scope
#[derive(Debug)]
pub struct CgroupV2 {
    /// Typically `/sys/fs/cgroup/screensteward.slice`.
    pub root: std::path::PathBuf,
}

impl CgroupV2 {
    #[must_use]
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn scope_dir(&self, id: &AppScopeId) -> std::path::PathBuf {
        self.root.join(format!("{}.scope", id.0))
    }
}

impl CgroupBackend for CgroupV2 {
    fn ensure_scope(&self, id: &AppScopeId) -> Result<()> {
        let d = self.scope_dir(id);
        if !d.exists() {
            std::fs::create_dir_all(&d)?;
        }
        Ok(())
    }

    fn move_pid(&self, id: &AppScopeId, pid: i32) -> Result<()> {
        let f = self.scope_dir(id).join("cgroup.procs");
        std::fs::write(&f, format!("{pid}\n"))?;
        Ok(())
    }

    fn freeze(&self, id: &AppScopeId) -> Result<()> {
        let f = self.scope_dir(id).join("cgroup.freeze");
        std::fs::write(&f, "1\n")?;
        Ok(())
    }

    fn unfreeze(&self, id: &AppScopeId) -> Result<()> {
        let f = self.scope_dir(id).join("cgroup.freeze");
        std::fs::write(&f, "0\n")?;
        Ok(())
    }

    fn kill_scope(&self, id: &AppScopeId) -> Result<()> {
        let f = self.scope_dir(id).join("cgroup.kill");
        std::fs::write(&f, "1\n")?;
        Ok(())
    }

    fn scope_exists(&self, id: &AppScopeId) -> bool {
        self.scope_dir(id).exists()
    }

    fn is_frozen(&self, id: &AppScopeId) -> bool {
        let f = self.scope_dir(id).join("cgroup.freeze");
        std::fs::read_to_string(&f).is_ok_and(|s| s.trim() == "1")
    }
}

/// Async trait reserved for a future version that may need async I/O. In
/// Phase 1 the cgroup writes are synchronous and fast, so this trait is
/// declared but not implemented.
#[async_trait]
pub trait CgroupBackendAsync: Send + Sync {
    /// # Errors
    /// Returns an error if the scope directory cannot be created.
    async fn ensure_scope(&self, id: &AppScopeId) -> Result<()>;
    /// # Errors
    /// Returns an error if writing the PID to `cgroup.procs` fails.
    async fn move_pid(&self, id: &AppScopeId, pid: i32) -> Result<()>;
    /// # Errors
    /// Returns an error if writing to `cgroup.freeze` fails.
    async fn freeze(&self, id: &AppScopeId) -> Result<()>;
    /// # Errors
    /// Returns an error if writing to `cgroup.freeze` fails.
    async fn unfreeze(&self, id: &AppScopeId) -> Result<()>;
    /// # Errors
    /// Returns an error if writing to `cgroup.kill` fails.
    async fn kill_scope(&self, id: &AppScopeId) -> Result<()>;
}
