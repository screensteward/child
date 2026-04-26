use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::PathBuf;
use std::process::Command;

use crate::errors::{Result, StoreError};

/// Abstraction de récupération de la clé maître `SQLCipher`.
pub trait Keyring: Send + Sync {
    /// Charge la clé si elle existe, sinon en génère une et la persiste.
    /// Renvoie les 32 octets bruts.
    ///
    /// # Errors
    ///
    /// Returns a [`StoreError`] if the key cannot be loaded or created.
    fn load_or_create(&self) -> Result<[u8; 32]>;
}

/// Clé stockée dans un fichier en clair, owner root 0600. Fallback quand
/// systemd-creds / TPM 2.0 n'est pas disponible (§9.1 spec Phase 1).
#[derive(Debug, Clone)]
pub struct FallbackKeyring {
    path: PathBuf,
}

impl FallbackKeyring {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Keyring for FallbackKeyring {
    fn load_or_create(&self) -> Result<[u8; 32]> {
        if self.path.exists() {
            let mut f = File::open(&self.path)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            if buf.len() != 32 {
                return Err(StoreError::InvalidKeyLength(buf.len()));
            }
            let mut out = [0u8; 32];
            out.copy_from_slice(&buf);
            Ok(out)
        } else {
            if let Some(parent) = self.path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut rng = [0u8; 32];
            getrandom(&mut rng)?;
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&self.path)?;
            f.write_all(&rng)?;
            f.sync_all()?;
            std::fs::set_permissions(&self.path, std::fs::Permissions::from_mode(0o600))?;
            Ok(rng)
        }
    }
}

/// Clé stockée scellée via `systemd-creds` (TPM 2.0 quand disponible).
/// On délègue complètement à l'outil — pas de dépendance libc sur le TPM.
#[derive(Debug, Clone)]
pub struct SystemdCredsKeyring {
    /// Nom logique (ex. "screensteward-master-key").
    pub name: String,
    /// Chemin où stocker le secret scellé (ex. `/var/lib/screensteward/master.cred`).
    pub sealed_path: PathBuf,
}

impl Keyring for SystemdCredsKeyring {
    fn load_or_create(&self) -> Result<[u8; 32]> {
        if self.sealed_path.exists() {
            // systemd-creds decrypt <sealed> -
            let out = Command::new("systemd-creds")
                .arg("decrypt")
                .arg(format!("--name={}", self.name))
                .arg(&self.sealed_path)
                .arg("-")
                .output()?;
            if !out.status.success() {
                return Err(StoreError::Io(std::io::Error::other(format!(
                    "systemd-creds decrypt failed: {}",
                    String::from_utf8_lossy(&out.stderr)
                ))));
            }
            if out.stdout.len() != 32 {
                return Err(StoreError::InvalidKeyLength(out.stdout.len()));
            }
            let mut k = [0u8; 32];
            k.copy_from_slice(&out.stdout);
            Ok(k)
        } else {
            if let Some(p) = self.sealed_path.parent() {
                std::fs::create_dir_all(p)?;
            }
            let mut rng = [0u8; 32];
            getrandom(&mut rng)?;
            // systemd-creds encrypt --name=<name> --with-key=auto - <sealed>
            let mut child = Command::new("systemd-creds")
                .arg("encrypt")
                .arg(format!("--name={}", self.name))
                .arg("--with-key=auto")
                .arg("-")
                .arg(&self.sealed_path)
                .stdin(std::process::Stdio::piped())
                .spawn()?;
            {
                let stdin = child.stdin.as_mut().unwrap();
                stdin.write_all(&rng)?;
            }
            let status = child.wait()?;
            if !status.success() {
                return Err(StoreError::Io(std::io::Error::other(
                    "systemd-creds encrypt failed",
                )));
            }
            Ok(rng)
        }
    }
}

/// Try `SystemdCreds` first, fallback on file if TPM / systemd-creds absent.
#[derive(Debug, Clone)]
pub struct LinuxKeyring {
    pub systemd_creds: SystemdCredsKeyring,
    pub fallback: FallbackKeyring,
}

impl Keyring for LinuxKeyring {
    fn load_or_create(&self) -> Result<[u8; 32]> {
        // Si le cred scellé existe déjà, on DOIT pouvoir le relire — sinon la DB
        // (encryptée avec la clé du cred) devient irrécupérable. Basculer sur
        // fallback à cet instant générerait une clé fraîche qui écraserait
        // silencieusement une clé utilisable. On refuse explicitement. Cf. bug
        // #7 dogfood 2026-04-24.
        if self.systemd_creds.sealed_path.exists() {
            return self
                .systemd_creds
                .load_or_create()
                .map_err(|e| StoreError::SealedCredUnreadable(Box::new(e)));
        }
        // Pas de cred scellé : soit première install, soit systemd-creds n'a
        // jamais été provisionné. On tente systemd-creds (qui créera le
        // sealed), et on accepte un fallback fichier clé si l'outil n'est pas
        // disponible sur cette machine.
        match self.systemd_creds.load_or_create() {
            Ok(k) => Ok(k),
            Err(e) => {
                tracing::warn!(error = %e, "systemd-creds indisponible, fallback sur fichier clé");
                self.fallback.load_or_create()
            }
        }
    }
}

fn getrandom(buf: &mut [u8]) -> Result<()> {
    // On évite d'ajouter une dépendance : on lit /dev/urandom.
    let mut f = File::open("/dev/urandom")?;
    f.read_exact(buf)?;
    Ok(())
}
