use ss_core_ipc::framing::{read_frame, write_frame};
use tokio::io::{duplex, BufReader};

#[tokio::test]
async fn frame_roundtrip() {
    let (mut a, b) = duplex(1024);
    let payload = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"ping","params":{}});
    write_frame(&mut a, &payload).await.unwrap();
    let mut br = BufReader::new(b);
    let got = read_frame(&mut br).await.unwrap().unwrap();
    assert_eq!(got, payload);
}

#[tokio::test]
async fn frame_eof_returns_none() {
    let (a, b) = duplex(128);
    drop(a);
    let mut br = BufReader::new(b);
    let got = read_frame(&mut br).await.unwrap();
    assert!(got.is_none());
}
