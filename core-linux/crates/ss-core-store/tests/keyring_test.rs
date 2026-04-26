use ss_core_store::keyring::{FallbackKeyring, Keyring, LinuxKeyring, SystemdCredsKeyring};
use tempfile::tempdir;

#[test]
fn fallback_generates_persists_and_returns_same_key() {
    let d = tempdir().unwrap();
    let path = d.path().join("master.key");
    let k1 = FallbackKeyring::new(path.clone()).load_or_create().unwrap();
    let k2 = FallbackKeyring::new(path).load_or_create().unwrap();
    assert_eq!(k1, k2);
    assert_eq!(k1.len(), 32);
}

#[test]
fn fallback_key_file_has_strict_perms() {
    use std::os::unix::fs::MetadataExt;
    let d = tempdir().unwrap();
    let path = d.path().join("master.key");
    let _ = FallbackKeyring::new(path.clone()).load_or_create().unwrap();
    let md = std::fs::metadata(&path).unwrap();
    let mode = md.mode() & 0o777;
    assert_eq!(mode, 0o600, "expected 0600, got 0o{mode:o}");
}

#[test]
fn fallback_rejects_wrong_size_file() {
    use std::io::Write;
    let d = tempdir().unwrap();
    let path = d.path().join("master.key");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(b"short").unwrap();
    let err = FallbackKeyring::new(path).load_or_create().unwrap_err();
    assert!(err.to_string().contains("key length") || err.to_string().contains("InvalidKey"));
}

/// Regression test for bug #6 (dogfood Phase 1 day 1, 2026-04-24) : systemd-creds
/// encrypt stored the credential with the embedded name `screensteward-master-key`
/// but decrypt was called without --name, so recent systemd-creds versions refused
/// to open the file (*"Embedded credential name does not match filename"*). Each
/// boot silently fell back to the file keyring, which generated a fresh random key
/// and made the `SQLCipher` DB unreadable.
///
/// Requires systemd-creds on PATH. Ignored by default (runs in the CI test-root
/// job, cf. §14.1 spec Phase 1).
#[test]
#[ignore = "requires systemd-creds on PATH — run with `cargo nextest --run-ignored all`"]
fn systemd_creds_encrypt_decrypt_round_trip() {
    let d = tempdir().unwrap();
    let sealed = d.path().join("master.cred");
    let k = SystemdCredsKeyring {
        name: "screensteward-master-key".into(),
        sealed_path: sealed.clone(),
    };
    let k1 = k.load_or_create().expect("first encrypt must succeed");
    assert!(sealed.exists(), "sealed file should have been written");
    let k2 = k
        .load_or_create()
        .expect("decrypt must succeed with --name override — bug #6 regression if this panics");
    assert_eq!(k1, k2, "round-trip must return the same 32-byte key");
    assert_eq!(k1.len(), 32);
}

/// Regression test for bug #7 (dogfood Phase 1 day 1, 2026-04-24) : when the
/// sealed credential file is present but cannot be decrypted (systemd upgrade
/// hardened the validation, TPM was swapped, file was corrupted, etc.), the
/// `LinuxKeyring` used to silently fall back to generating a fresh file key,
/// making the existing `SQLCipher` DB unrecoverable. The current behaviour must
/// be to **refuse to boot** instead, and leave the fallback file untouched.
///
/// Requires systemd-creds on PATH to exercise the decrypt failure path.
#[test]
#[ignore = "requires systemd-creds on PATH — run with `cargo test -- --ignored`"]
fn linux_keyring_refuses_fallback_when_sealed_is_unreadable() {
    let d = tempdir().unwrap();
    let sealed = d.path().join("master.cred");
    // Write garbage — systemd-creds decrypt must fail with "not a valid credential".
    std::fs::write(&sealed, b"not a real systemd-creds sealed file, at all").unwrap();
    let fallback_path = d.path().join("master.key");
    let k = LinuxKeyring {
        systemd_creds: SystemdCredsKeyring {
            name: "screensteward-master-key".into(),
            sealed_path: sealed.clone(),
        },
        fallback: FallbackKeyring::new(fallback_path.clone()),
    };
    let err = k
        .load_or_create()
        .expect_err("must refuse — sealed present but unreadable");
    assert!(
        err.to_string().contains("unreadable") || err.to_string().contains("SealedCredUnreadable"),
        "error should flag the sealed-cred path explicitly, got: {err}"
    );
    assert!(
        !fallback_path.exists(),
        "fallback file must NOT have been created — that would corrupt the existing DB"
    );
}
