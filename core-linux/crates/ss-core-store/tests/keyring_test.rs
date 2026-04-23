use ss_core_store::keyring::{FallbackKeyring, Keyring};
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
