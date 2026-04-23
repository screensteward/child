use serde_json::json;
use ss_core_ipc::rpc::Response;
use ss_core_ipc::server::{MethodContext, MethodHandler, Server};
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

struct PingHandler;

#[async_trait::async_trait]
impl MethodHandler for PingHandler {
    fn name(&self) -> &'static str {
        "ping"
    }
    fn requires_auth(&self) -> bool {
        false
    }
    async fn call(
        &self,
        _ctx: MethodContext<'_>,
        _params: serde_json::Value,
    ) -> ss_core_ipc::Result<serde_json::Value> {
        Ok(json!({"pong": true}))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn server_answers_ping() {
    let dir = tempdir().unwrap();
    let sock = dir.path().join("ss.sock");
    let mut handlers: HashMap<String, Arc<dyn MethodHandler>> = HashMap::new();
    handlers.insert("ping".into(), Arc::new(PingHandler));
    let server = Server::new(sock.clone(), handlers);
    let handle = tokio::spawn(async move { server.serve().await });

    // Give the server a moment to bind.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let stream = UnixStream::connect(&sock).await.unwrap();
    let (rd, mut wr) = stream.into_split();
    let mut br = BufReader::new(rd);
    let req = r#"{"jsonrpc":"2.0","id":1,"method":"ping","params":{}}"#.to_string() + "\n";
    wr.write_all(req.as_bytes()).await.unwrap();
    wr.flush().await.unwrap();

    let mut line = String::new();
    br.read_line(&mut line).await.unwrap();
    let resp: Response = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(resp.id, json!(1));
    assert_eq!(resp.result.unwrap()["pong"], json!(true));

    handle.abort();
}

struct SecretHandler;

#[async_trait::async_trait]
impl MethodHandler for SecretHandler {
    fn name(&self) -> &'static str {
        "secret"
    }
    fn requires_auth(&self) -> bool {
        true
    }
    async fn call(
        &self,
        _ctx: MethodContext<'_>,
        _params: serde_json::Value,
    ) -> ss_core_ipc::Result<serde_json::Value> {
        Ok(json!({"ok": true}))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn server_enforces_auth_on_protected_method() {
    let dir = tempdir().unwrap();
    let sock = dir.path().join("ss.sock");
    let mut h: HashMap<String, Arc<dyn MethodHandler>> = HashMap::new();
    h.insert("secret".into(), Arc::new(SecretHandler));
    let server = Server::new(sock.clone(), h);
    let handle = tokio::spawn(async move { server.serve().await });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let stream = UnixStream::connect(&sock).await.unwrap();
    let (rd, mut wr) = stream.into_split();
    let mut br = BufReader::new(rd);
    let req = r#"{"jsonrpc":"2.0","id":1,"method":"secret","params":{}}"#.to_string() + "\n";
    wr.write_all(req.as_bytes()).await.unwrap();
    wr.flush().await.unwrap();
    let mut line = String::new();
    br.read_line(&mut line).await.unwrap();
    let resp: Response = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(
        resp.error.as_ref().unwrap().code,
        ss_core_ipc::rpc::codes::UNAUTHORIZED
    );

    handle.abort();
}
