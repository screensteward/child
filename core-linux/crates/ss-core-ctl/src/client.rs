use serde_json::{json, Value};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub struct CtlClient {
    reader: BufReader<tokio::net::unix::OwnedReadHalf>,
    writer: tokio::net::unix::OwnedWriteHalf,
    next_id: u64,
}

impl std::fmt::Debug for CtlClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CtlClient")
            .field("next_id", &self.next_id)
            .finish_non_exhaustive()
    }
}

impl CtlClient {
    pub async fn connect(socket: &Path) -> anyhow::Result<Self> {
        let s = UnixStream::connect(socket).await?;
        let (r, w) = s.into_split();
        Ok(Self {
            reader: BufReader::new(r),
            writer: w,
            next_id: 1,
        })
    }

    pub async fn call(&mut self, method: &str, params: Value) -> anyhow::Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        let req = json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });
        let mut s = serde_json::to_string(&req)?;
        s.push('\n');
        self.writer.write_all(s.as_bytes()).await?;
        self.writer.flush().await?;
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        let resp: Value = serde_json::from_str(line.trim())?;
        if let Some(err) = resp.get("error") {
            anyhow::bail!("rpc error: {err}");
        }
        Ok(resp.get("result").cloned().unwrap_or(Value::Null))
    }
}
