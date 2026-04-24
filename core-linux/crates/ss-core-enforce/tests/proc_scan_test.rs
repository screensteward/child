use ss_core_enforce::proc_scan::{compute_content_hash, ContentHashCache, HashKey};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn content_hash_of_file_is_stable() {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(b"hello world").unwrap();
    let p = f.path().to_path_buf();
    let h1 = compute_content_hash(&p).unwrap();
    let h2 = compute_content_hash(&p).unwrap();
    assert_eq!(h1, h2);
    assert!(h1.starts_with("sha256:"));
}

#[test]
fn cache_returns_same_hash_for_same_inode_mtime_size() {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(b"abcdef").unwrap();
    let p = f.path().to_path_buf();
    let cache = ContentHashCache::new();
    let h1 = cache.get_or_compute(&p).unwrap();
    let h2 = cache.get_or_compute(&p).unwrap();
    assert_eq!(h1, h2);
}

#[test]
fn hash_key_differentiates_mtime() {
    let a = HashKey {
        inode: 1,
        mtime: 100,
        size: 42,
    };
    let b = HashKey {
        inode: 1,
        mtime: 101,
        size: 42,
    };
    assert_ne!(a, b);
}
