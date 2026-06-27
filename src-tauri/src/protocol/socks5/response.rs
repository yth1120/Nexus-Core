use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::utils::{AppError, AppResult};

/// A SOCKS5 response.
pub struct Socks5Response {
    pub reply: u8,
    pub bind_addr: [u8; 4],
    pub bind_port: u16,
}

impl Socks5Response {
    pub fn success() -> Self {
        Self {
            reply: 0,
            bind_addr: [0, 0, 0, 0],
            bind_port: 0,
        }
    }

    pub fn failure(reply: u8) -> Self {
        Self {
            reply,
            bind_addr: [0, 0, 0, 0],
            bind_port: 0,
        }
    }
}

/// Write a SOCKS5 response: VER | REP | RSV | ATYP | BND.ADDR | BND.PORT.
pub async fn write_response(stream: &mut TcpStream, resp: &Socks5Response) -> AppResult<()> {
    let mut buf = vec![5, resp.reply, 0, 1]; // VER, REP, RSV, ATYP=IPv4
    buf.extend_from_slice(&resp.bind_addr);
    buf.extend_from_slice(&resp.bind_port.to_be_bytes());
    stream
        .write_all(&buf)
        .await
        .map_err(|e| AppError::Io(format!("socks5: response write error: {e}")))
}
