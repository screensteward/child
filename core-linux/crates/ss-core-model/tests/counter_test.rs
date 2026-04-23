use ss_core_model::counter::GCounter;
use uuid::Uuid;

#[test]
fn gcounter_single_device_total() {
    let d = Uuid::new_v4();
    let mut c = GCounter::new();
    c.increment(d, 5);
    c.increment(d, 3);
    assert_eq!(c.total(), 8);
}

#[test]
fn gcounter_merge_is_max_per_key() {
    let d1 = Uuid::new_v4();
    let d2 = Uuid::new_v4();
    let mut a = GCounter::new();
    a.increment(d1, 10);
    a.increment(d2, 3);

    let mut b = GCounter::new();
    b.increment(d1, 7);
    b.increment(d2, 12);

    a.merge(&b);
    assert_eq!(a.total(), 10 + 12);
}

#[test]
fn gcounter_serializes_stable() {
    let d = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let mut c = GCounter::new();
    c.increment(d, 42);
    let j = serde_json::to_string(&c).unwrap();
    let back: GCounter = serde_json::from_str(&j).unwrap();
    assert_eq!(c, back);
}
