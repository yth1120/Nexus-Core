use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::connection::ConnectionTask;
use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};
use tokio::net::TcpListener;

use super::proxy_trait::{ProxyAdapter, ProxyKind};

/// Real SOCKS5 CONNECT proxy — binds TCP, spawns accept loop with
/// cancellation support.
pub struct Socks5ProxyAdapter {
    port: u16,
    context: Arc<RuntimeContext>,
    running: RwLock<bool>,
    cancel_token: RwLock<Option<tokio_util::sync::CancellationToken>>,
}

impl Socks5ProxyAdapter {
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
impl ProxyAdapter for Socks5ProxyAdapter {
    async fn start(&self) -> AppResult<()> {
        if *self.running.read() {
            return Ok(());
        }
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| AppError::Io(format!("SOCKS5 proxy bind {addr}: {e}")))?;

        *self.running.write() = true;

        let emitter = self.context.emitter.clone();
        let token = self.context.shutdown_token.child();
        *self.cancel_token.write() = Some(token.clone());
        emitter.emit(AppEvent::ProxyStarted {
            kind: "socks5".into(),
        });
        tracing::info!("SOCKS5 proxy listening on {addr}");

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled() => break,
                    result = listener.accept() => {
                        match result {
                            Ok((stream, addr)) => {
                                let e = emitter.clone();
                                tokio::spawn(async move {
                                    match ConnectionTask::handle_socks5(stream, addr, e).await {
                                        Ok((sent, recv)) => {
                                            tracing::debug!("SOCKS5 conn {}: sent={sent} recv={recv}", addr);
                                        }
                                        Err(err) => tracing::debug!("SOCKS5 conn error: {err}"),
                                    }
                                });
                            }
                            Err(e) => tracing::warn!("SOCKS5 accept error: {e}"),
                        }
                    }
                }
            }
            tracing::info!("SOCKS5 proxy accept loop exited");
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
            kind: "socks5".into(),
        });
        tracing::info!("SOCKS5 proxy stopped");
        Ok(())
    }

    fn kind(&self) -> ProxyKind {
        ProxyKind::Socks5
    }
}
