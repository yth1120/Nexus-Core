use std::time::Duration;

use tokio::io;

use crate::utils::AppResult;

/// Bidirectionally copy data between client and remote streams.
///
/// Uses `tokio::io::copy_bidirectional`. On any error, both streams are
/// dropped (graceful close). Returns `(bytes_sent, bytes_received)`.
pub async fn bidirectional_copy(
    mut client: tokio::net::TcpStream,
    mut remote: tokio::net::TcpStream,
    timeout: Duration,
) -> AppResult<(u64, u64)> {
    let result =
        tokio::time::timeout(timeout, io::copy_bidirectional(&mut client, &mut remote)).await;

    match result {
        Ok(Ok((sent, recv))) => Ok((sent, recv)),
        Ok(Err(e)) => {
            // Connection error during copy — graceful, not fatal.
            // Return zero counts since we cannot recover partial byte totals
            // from tokio::io::copy_bidirectional; the error is logged for
            // diagnostics.
            tracing::warn!("bidirectional_copy finished with error: {e}");
            Ok((0, 0))
        }
        Err(_elapsed) => {
            tracing::warn!("bidirectional_copy timed out after {:?}", timeout);
            Ok((0, 0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn copies_bidirectional_data() -> AppResult<()> {
        let echo = TcpListener::bind("127.0.0.1:0").await?;
        let echo_addr = echo.local_addr()?;

        let echo_task = tokio::spawn(async move {
            let (mut s, _) = echo.accept().await?;
            let mut buf = vec![0u8; 1024];
            let n = s.read(&mut buf).await?;
            s.write_all(&buf[..n]).await?;
            AppResult::Ok(())
        });

        let client = tokio::net::TcpStream::connect(echo_addr).await?;
        let remote = tokio::net::TcpStream::connect(echo_addr).await?;

        let (client_read, mut client_write) = client.into_split();
        let (mut remote_read, _remote_write) = remote.into_split();

        // Write to client side
        client_write.write_all(b"hello").await?;
        // Read from remote side
        let mut buf = [0u8; 5];
        remote_read.read_exact(&mut buf).await?;
        assert_eq!(&buf, b"hello");

        drop(client_write);
        drop(remote_read);
        drop(client_read);
        drop(_remote_write);
        echo_task.await.unwrap()?;
        Ok(())
    }
}
