use serde_json::json;
use ss_core_ipc::rpc::{Request, Response};

#[test]
fn request_parses_with_params() {
    let raw = r#"{"jsonrpc":"2.0","id":42,"method":"ping","params":{"x":1}}"#;
    let req: Request = serde_json::from_str(raw).unwrap();
    assert_eq!(req.method, "ping");
    assert_eq!(req.id, Some(serde_json::Value::from(42)));
    assert_eq!(req.params["x"], 1);
}

#[test]
fn response_ok_serializes() {
    let r = Response::ok(json!(1), json!({"pong": true}));
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.contains("\"result\""));
    assert!(!s.contains("\"error\""));
}

#[test]
fn response_err_serializes_with_code() {
    let r = Response::err(json!(1), -32601, "method not found");
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.contains("\"code\":-32601"));
}
