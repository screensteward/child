use ss_core_ipc::auth::{hash_password, verify_password};

#[test]
fn hash_and_verify_round_trip() {
    let h = hash_password("correct horse").unwrap();
    assert!(h.starts_with("$argon2id$"));
    assert!(verify_password("correct horse", &h).unwrap());
    assert!(!verify_password("wrong", &h).unwrap());
}

#[test]
fn rejects_malformed_hash() {
    let e = verify_password("x", "not-argon").unwrap_err();
    assert!(e.to_string().contains("argon2"));
}
