use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::errors::{IpcError, Result};

/// Read a single ND-JSON frame (one newline-terminated line).
///
/// Returns `None` on clean EOF (zero bytes read).
///
/// # Errors
///
/// Returns [`IpcError::Framing`] if the line is empty (bare `\n`).
/// Returns [`IpcError::Json`] if the line is not valid JSON.
/// Returns [`IpcError::Io`] on I/O failure.
pub async fn read_frame<R>(reader: &mut BufReader<R>) -> Result<Option<Value>>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut buf = String::new();
    let n = reader.read_line(&mut buf).await?;
    if n == 0 {
        return Ok(None);
    }
    let trimmed = buf.trim_end_matches(['\r', '\n']);
    if trimmed.is_empty() {
        return Err(IpcError::Framing("empty frame".into()));
    }
    let v: Value = serde_json::from_str(trimmed)?;
    Ok(Some(v))
}

/// Write a single ND-JSON frame (JSON value followed by `\n`) and flush.
///
/// # Errors
///
/// Returns [`IpcError::Json`] if serialization fails.
/// Returns [`IpcError::Io`] on I/O failure.
pub async fn write_frame<W>(writer: &mut W, value: &Value) -> Result<()>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    let mut s = serde_json::to_string(value)?;
    s.push('\n');
    writer.write_all(s.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}
