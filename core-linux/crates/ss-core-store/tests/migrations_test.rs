use ss_core_store::Store;
use tempfile::tempdir;

#[test]
fn migrations_apply_on_fresh_db() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("core.db");
    let key = [0u8; 32];
    let store = Store::open_with_key(&path, &key).unwrap();
    // Toutes les tables doivent exister.
    let conn = store.conn_for_test();
    let row_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'refinery_%'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(row_count >= 9, "expected >= 9 tables, got {row_count}");
}

#[test]
fn reopening_with_wrong_key_fails() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("core.db");
    let mut key_ok = [0u8; 32];
    key_ok[0] = 1;
    {
        let _ = ss_core_store::Store::open_with_key(&path, &key_ok).unwrap();
    }
    let mut key_bad = [0u8; 32];
    key_bad[0] = 2;
    let err = ss_core_store::Store::open_with_key(&path, &key_bad).unwrap_err();
    // L'erreur exacte dépend de SQLCipher, on vérifie juste qu'on échoue proprement.
    let s = err.to_string();
    assert!(s.contains("sqlite") || s.contains("not a database") || s.contains("encrypted"));
}
