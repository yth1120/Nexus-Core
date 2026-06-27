use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::event::BackendEmitter;
use crate::protocol::http::{parse_connect_request, ProxyResponse};
use crate::protocol::socks5;
use crate::proxy::tunnel;
use crate::utils::{AppError, AppResult};

/// Per-connection handler spawned by the proxy accept loop.
pub struct ConnectionTask;

/// Maximum time allowed for the connection handshake phase.
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);

impl ConnectionTask {
    /// Handle a single HTTP CONNECT connection.
    pub async fn handle_http(
        mut client: tokio::net::TcpStream,
        _addr: std::net::SocketAddr,
        _emitter: Arc<dyn BackendEmitter>,
    ) -> AppResult<(u64, u64)> {
        let mut buf = vec![0u8; 8192];
        let n = tokio::time::timeout(HANDSHAKE_TIMEOUT, client.read(&mut buf))
            .await
            .map_err(|_| AppError::Io("http handshake read timed out".into()))?
            .map_err(|e| AppError::Io(format!("http proxy read: {e}")))?;
        if n == 0 {
            return Ok((0, 0));
        }
        buf.truncate(n);

        let req = parse_connect_request(&buf)?;
        let remote_addr = format!("{}:{}", req.host, req.port);
        let remote = tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            tokio::net::TcpStream::connect(&remote_addr),
        )
        .await
        .map_err(|_| AppError::Io(format!("connect {remote_addr} timed out")))?
        .map_err(|e| AppError::Io(format!("connect {remote_addr}: {e}")))?;

        tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            client.write_all(&ProxyResponse::ok().to_bytes()),
        )
        .await
        .map_err(|_| AppError::Io("http write 200 timed out".into()))?
        .map_err(|e| AppError::Io(format!("http write 200: {e}")))?;

        tunnel::bidirectional_copy(client, remote, Duration::from_secs(300)).await
    }

    /// Handle a single SOCKS5 CONNECT connection.
    pub async fn handle_socks5(
        mut client: tokio::net::TcpStream,
        _addr: std::net::SocketAddr,
        _emitter: Arc<dyn BackendEmitter>,
    ) -> AppResult<(u64, u64)> {
        // SOCKS5 handshake with per-phase timeouts
        tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            socks5::handshake::read_handshake(&mut client),
        )
        .await
        .map_err(|_| AppError::Io("socks5 handshake read timed out".into()))??;
        tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            socks5::handshake::write_handshake(&mut client),
        )
        .await
        .map_err(|_| AppError::Io("socks5 handshake write timed out".into()))??;

        let request = tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            socks5::request::read_request(&mut client),
        )
        .await
        .map_err(|_| AppError::Io("socks5 request read timed out".into()))??;

        let remote_addr = format!("{}:{}", request.addr, request.port);
        let remote = tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            tokio::net::TcpStream::connect(&remote_addr),
        )
        .await
        .map_err(|_| AppError::Io(format!("socks5 connect {remote_addr} timed out")))?
        .map_err(|e| AppError::Io(format!("socks5 connect {remote_addr}: {e}")))?;

        tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            socks5::response::write_response(
                &mut client,
                &socks5::response::Socks5Response::success(),
            ),
        )
        .await
        .map_err(|_| AppError::Io("socks5 response write timed out".into()))??;

        tunnel::bidirectional_copy(client, remote, Duration::from_secs(300)).await
    }
}
