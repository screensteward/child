use std::path::Path;
use std::sync::Arc;

use parking_lot::Mutex;
use rusqlite::Connection;

use crate::errors::{Result, StoreError};

mod embedded {
    refinery::embed_migrations!("migrations");
}

/// Handle partagée sur la DB `SQLCipher`. Thread-safe via un mutex
/// (`SQLite` n'aime pas le multi-writer concurrent, un seul writer en
/// Phase 1 est suffisant ; on passera à un pool si besoin).
#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<Connection>>,
}

impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store").finish_non_exhaustive()
    }
}

impl Store {
    /// Ouvre (ou crée) la DB chiffrée avec `key` (32 octets). Applique les migrations.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Key length is not 32 bytes
    /// - `SQLite` connection fails
    /// - `SQLCipher` decryption fails with wrong key
    /// - Migration runner fails
    pub fn open_with_key(path: &Path, key: &[u8]) -> Result<Self> {
        if key.len() != 32 {
            return Err(StoreError::InvalidKeyLength(key.len()));
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut conn = Connection::open(path)?;
        // PRAGMA key en hex pour éviter les problèmes d'échappement.
        let hex = hex::encode(key);
        conn.execute_batch(&format!(r#"PRAGMA key = "x'{hex}'";"#))?;
        // Touch pour vérifier la clé valide avant de continuer.
        conn.execute_batch("SELECT count(*) FROM sqlite_master;")?;
        // Migrations.
        embedded::migrations::runner().run(&mut conn)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(conn)),
        })
    }

    /// Exécute une closure avec un accès mut à la connexion.
    ///
    /// # Errors
    ///
    /// Returns an error if the closure returns an error.
    pub fn with_conn<R>(&self, f: impl FnOnce(&mut Connection) -> Result<R>) -> Result<R> {
        let mut guard = self.inner.lock();
        f(&mut guard)
    }

    /// Réservé aux tests et au diagnostic via CLI.
    pub fn conn_for_test(&self) -> parking_lot::MutexGuard<'_, Connection> {
        self.inner.lock()
    }
}
