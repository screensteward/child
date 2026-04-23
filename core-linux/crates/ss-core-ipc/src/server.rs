//! Unix socket JSON-RPC 2.0 server.
//!
//! One tokio task per connection. Each connection owns:
//! * a per-FD [`crate::auth::ConnState`] (dropped on EOF),
//! * an `mpsc` write channel funnelling all outbound frames through a single
//!   writer task (prevents interleaved `write_frame` calls which would
//!   corrupt the ND-JSON framing),
//! * a `broadcast` receiver on the shared [`NotificationEmitter`], filtered
//!   by the connection's subscribed topics.

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::auth::ConnState;
use crate::errors::{IpcError, Result};
use crate::framing::{read_frame, write_frame};
use crate::rpc::{codes, Request, Response};

/// Context passed to every [`MethodHandler::call`].
#[derive(Debug)]
pub struct MethodContext<'a> {
    pub conn_state: &'a ConnState,
    pub notifier: &'a NotificationEmitter,
}

/// Server-side method implementation.
#[async_trait]
pub trait MethodHandler: Send + Sync {
    fn name(&self) -> &'static str;
    fn requires_auth(&self) -> bool;
    /// Invoke the method.
    ///
    /// # Errors
    ///
    /// Returns an [`IpcError`] which the server maps to the appropriate
    /// JSON-RPC error code (`Unauthorized` → `-32000`, `InvalidParams` →
    /// `-32602`, `MethodNotFound` → `-32601`, anything else → `-32603`).
    async fn call(&self, ctx: MethodContext<'_>, params: Value) -> Result<Value>;
}

/// Broadcast fan-out for server-initiated notifications. Cheap to clone.
#[derive(Debug, Clone)]
pub struct NotificationEmitter {
    tx: broadcast::Sender<(String, Value)>,
}

impl NotificationEmitter {
    /// Build a fresh emitter with a 256-slot per-subscriber buffer.
    #[must_use]
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(256);
        Self { tx }
    }

    /// Subscribe to future notifications. Messages emitted before this call
    /// are not replayed.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<(String, Value)> {
        self.tx.subscribe()
    }

    /// Emit a notification. Silently swallows "no subscribers" — it is not
    /// an error for the server to publish while nobody is listening.
    pub fn emit(&self, topic: impl Into<String>, params: Value) {
        let _ = self.tx.send((topic.into(), params));
    }
}

impl Default for NotificationEmitter {
    fn default() -> Self {
        Self::new()
    }
}

/// IPC server.
pub struct Server {
    sock_path: PathBuf,
    handlers: Arc<HashMap<String, Arc<dyn MethodHandler>>>,
    emitter: NotificationEmitter,
}

impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Server")
            .field("sock_path", &self.sock_path)
            .field("handler_count", &self.handlers.len())
            .field("emitter", &self.emitter)
            .finish()
    }
}

impl Server {
    /// Build a server with a fresh emitter.
    #[must_use]
    pub fn new(sock_path: PathBuf, handlers: HashMap<String, Arc<dyn MethodHandler>>) -> Self {
        Self {
            sock_path,
            handlers: Arc::new(handlers),
            emitter: NotificationEmitter::new(),
        }
    }

    /// Build a server sharing an existing emitter. Used by the daemon, where
    /// the emitter must exist before handlers (Tasks 16–18) so they can emit
    /// through it.
    #[must_use]
    pub fn with_emitter(
        sock_path: PathBuf,
        handlers: HashMap<String, Arc<dyn MethodHandler>>,
        emitter: NotificationEmitter,
    ) -> Self {
        Self {
            sock_path,
            handlers: Arc::new(handlers),
            emitter,
        }
    }

    /// A clone of the server's notification emitter.
    #[must_use]
    pub fn emitter(&self) -> NotificationEmitter {
        self.emitter.clone()
    }

    /// Bind the Unix socket and run the accept loop until an I/O error.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::Io`] if bind/accept fails.
    pub async fn serve(self) -> Result<()> {
        // Clean up a stale socket from a previous run.
        let _ = std::fs::remove_file(&self.sock_path);
        let listener = UnixListener::bind(&self.sock_path)?;
        info!(path = ?self.sock_path, "IPC listener started");

        loop {
            let (stream, _) = listener.accept().await?;
            let handlers = self.handlers.clone();
            let emitter = self.emitter.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_conn(stream, handlers, emitter).await {
                    warn!(error = %e, "connection handler ended with error");
                }
            });
        }
    }
}

/// Per-connection handler: spawns a writer task + a notification fan-out
/// task, then runs the request/response pipeline on the reader.
///
/// # Errors
///
/// Returns [`IpcError::Io`] on read failure, [`IpcError::Json`] on
/// serialization failure.
async fn handle_conn(
    stream: UnixStream,
    handlers: Arc<HashMap<String, Arc<dyn MethodHandler>>>,
    emitter: NotificationEmitter,
) -> Result<()> {
    let (rd, mut wr) = stream.into_split();
    let mut br = tokio::io::BufReader::new(rd);
    let conn_state = ConnState::new();

    // Single writer: every outbound frame (responses + notifications) flows
    // through this mpsc channel so we never interleave writes on `wr`.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Value>(128);

    // Fan-out task: drains the broadcast receiver, filters by the
    // connection's subscribed topics, re-formats as a JSON-RPC notification
    // and pushes to the writer channel.
    let conn_state_for_sub = conn_state.clone();
    let mut notif_rx = emitter.subscribe();
    let notif_tx = tx.clone();
    tokio::spawn(async move {
        while let Ok((topic, params)) = notif_rx.recv().await {
            if !conn_state_for_sub.is_subscribed(&topic) {
                continue;
            }
            let frame = serde_json::json!({
                "jsonrpc": "2.0",
                "method": format!("on{}", cap(&topic)),
                "params": params,
            });
            if notif_tx.send(frame).await.is_err() {
                break;
            }
        }
    });

    // Writer task: pops frames and pushes them to the socket.
    let writer_task = tokio::spawn(async move {
        while let Some(v) = rx.recv().await {
            if let Err(e) = write_frame(&mut wr, &v).await {
                debug!(error = %e, "connection closed during write");
                break;
            }
        }
    });

    // Reader / dispatcher loop.
    loop {
        let Some(frame) = read_frame(&mut br).await? else {
            break;
        };
        let req: Request = match serde_json::from_value(frame.clone()) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response::err(
                    Value::Null,
                    codes::INVALID_REQUEST,
                    format!("invalid request: {e}"),
                );
                let _ = tx.send(serde_json::to_value(resp)?).await;
                continue;
            }
        };

        let id = req.id.clone().unwrap_or(Value::Null);
        let method = req.method.clone();
        let params = req.params.clone();

        let resp = match handlers.get(&method) {
            None => Response::err(
                id.clone(),
                codes::METHOD_NOT_FOUND,
                format!("method '{method}' not found"),
            ),
            Some(h) => {
                if h.requires_auth() && !conn_state.is_authenticated() {
                    Response::err(id.clone(), codes::UNAUTHORIZED, "authentication required")
                } else {
                    let ctx = MethodContext {
                        conn_state: &conn_state,
                        notifier: &emitter,
                    };
                    match h.call(ctx, params).await {
                        Ok(v) => Response::ok(id.clone(), v),
                        Err(IpcError::Unauthorized) => {
                            Response::err(id.clone(), codes::UNAUTHORIZED, "unauthorized")
                        }
                        Err(IpcError::InvalidParams(m)) => {
                            Response::err(id.clone(), codes::INVALID_PARAMS, m)
                        }
                        Err(IpcError::MethodNotFound(m)) => {
                            Response::err(id.clone(), codes::METHOD_NOT_FOUND, m)
                        }
                        Err(e) => {
                            error!(method = %method, error = %e, "handler failed");
                            Response::err(id.clone(), codes::INTERNAL_ERROR, e.to_string())
                        }
                    }
                }
            }
        };

        if tx.send(serde_json::to_value(resp)?).await.is_err() {
            break;
        }
    }

    // Close the writer channel; wait for the writer task to drain and exit.
    // The fan-out task will notice the channel closure and exit on its own.
    drop(tx);
    let _ = writer_task.await;
    Ok(())
}

/// Capitalize the first character of `s` (ASCII or Unicode).
fn cap(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
