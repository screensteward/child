use ss_core_enforce::idle::{IdleDetector, IdleSource};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct FakeIdle {
    idle: Arc<AtomicBool>,
}

#[async_trait::async_trait]
impl IdleSource for FakeIdle {
    async fn is_idle(&self) -> Result<bool, ss_core_enforce::EnforceError> {
        Ok(self.idle.load(Ordering::SeqCst))
    }
    fn name(&self) -> &'static str {
        "fake"
    }
}

#[tokio::test]
async fn idle_detector_reflects_source() {
    let flag = Arc::new(AtomicBool::new(false));
    let det = IdleDetector::new(vec![Box::new(FakeIdle { idle: flag.clone() })]);
    assert!(!det.is_idle_now().await.unwrap());
    flag.store(true, Ordering::SeqCst);
    assert!(det.is_idle_now().await.unwrap());
}

#[tokio::test]
async fn idle_detector_idle_if_any_source_idle() {
    let f1 = Arc::new(AtomicBool::new(false));
    let f2 = Arc::new(AtomicBool::new(true));
    let det = IdleDetector::new(vec![
        Box::new(FakeIdle { idle: f1 }),
        Box::new(FakeIdle { idle: f2 }),
    ]);
    // any source reporting idle -> global idle
    assert!(det.is_idle_now().await.unwrap());
}
