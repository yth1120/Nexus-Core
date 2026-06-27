use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::utils::{AppError, AppResult};

// SOCKS5 constants
const VER: u8 = 5;
const METHOD_NO_AUTH: u8 = 0;

/// Read the SOCKS5 handshake from the client. Validates VER=5 and that
/// NO AUTHENTICATION (0x00) is offered.
pub async fn read_handshake(stream: &mut TcpStream) -> AppResult<()> {
    let mut buf = [0u8; 2];
    stream
        .read_exact(&mut buf)
        .await
        .map_err(|e| AppError::Validation(format!("socks5: handshake read error: {e}")))?;

    if buf[0] != VER {
        return Err(AppError::Validation(format!(
            "socks5: unsupported version {}",
            buf[0]
        )));
    }

    let nmethods = buf[1] as usize;
    let mut methods = vec![0u8; nmethods];
    stream
        .read_exact(&mut methods)
        .await
        .map_err(|e| AppError::Validation(format!("socks5: methods read error: {e}")))?;

    if !methods.contains(&METHOD_NO_AUTH) {
        return Err(AppError::Validation("socks5: NO AUTH not offered".into()));
    }

    Ok(())
}

/// Write the SOCKS5 handshake response: VER=5, METHOD=0 (NO AUTH).
pub async fn write_handshake(stream: &mut TcpStream) -> AppResult<()> {
    stream
        .write_all(&[VER, METHOD_NO_AUTH])
        .await
        .map_err(|e| AppError::Io(format!("socks5: handshake write error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handshake_round_trip() -> AppResult<()> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await?;
            read_handshake(&mut s).await?;
            write_handshake(&mut s).await?;
            AppResult::Ok(())
        });

        let mut client = TcpStream::connect(addr).await?;
        // Send: VER=5, NMETHODS=1, METHODS=[0x00]
        client.write_all(&[5, 1, 0]).await?;
        let mut resp = [0u8; 2];
        client.read_exact(&mut resp).await?;
        assert_eq!(resp, [5, 0]);
        server.await.unwrap()?;
        Ok(())
    }
}
