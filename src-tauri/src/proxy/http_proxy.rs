use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::connection::ConnectionTask;
use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};
use tokio::net::TcpListener;

use super::proxy_trait::{ProxyAdapter, ProxyKind};

/// Real HTTP CONNECT proxy — binds TCP, spawns accept loop with
/// cancellation support.
pub struct HttpProxyAdapter {
    port: u16,
    context: Arc<RuntimeContext>,
    running: RwLock<bool>,
    cancel_token: RwLock<Option<tokio_util::sync::CancellationToken>>,
}

impl HttpProxyAdapter {
    pub fn new(port: u16, context: Arc<RuntimeContext>) -> Self {
        Self {
            port,
            context,
            running: RwLock::new(false),
            cancel_token: RwLock::new(None),
        }
    }

    pub fn is_running(&self) -> bool {
        *self.running.read()
    }
}

#[async_trait]
impl ProxyAdapter for HttpProxyAdapter {
    async fn start(&self) -> AppResult<()> {
        if *self.running.read() {
            return Ok(());
        }
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| AppError::Io(format!("HTTP proxy bind {addr}: {e}")))?;

        *self.running.write() = true;

        let emitter = self.context.emitter.clone();
        let token = self.context.shutdown_token.child();
        *self.cancel_token.write() = Some(token.clone());
        emitter.emit(AppEvent::ProxyStarted {
            kind: "http".into(),
        });
        tracing::info!("HTTP proxy listening on {addr}");

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled() => break,
                    result = listener.accept() => {
                        match result {
                            Ok((stream, addr)) => {
                                let e = emitter.clone();
                                tokio::spawn(async move {
                                    match ConnectionTask::handle_http(stream, addr, e).await {
                                        Ok((sent, recv)) => {
                                            tracing::debug!("HTTP conn {}: sent={sent} recv={recv}", addr);
                                        }
                                        Err(err) => tracing::debug!("HTTP conn error: {err}"),
                                    }
                                });
                            }
                            Err(e) => tracing::warn!("HTTP accept error: {e}"),
                        }
                    }
                }
            }
            tracing::info!("HTTP proxy accept loop exited");
        });

        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        if !*self.running.read() {
            return Ok(());
        }
        if let Some(ref t) = *self.cancel_token.read() {
            t.cancel();
        }
        *self.running.write() = false;
        self.context.emitter.emit(AppEvent::ProxyStopped {
            kind: "http".into(),
        });
        tracing::info!("HTTP proxy stopped");
        Ok(())
    }

    fn kind(&self) -> ProxyKind {
        ProxyKind::Http
    }
}
