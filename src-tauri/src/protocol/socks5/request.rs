use std::net::Ipv4Addr;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::utils::{AppError, AppResult};

/// A parsed SOCKS5 CONNECT request.
#[derive(Debug, Clone)]
pub struct Socks5Request {
    pub atyp: u8,
    pub addr: String,
    pub port: u16,
}

/// Read a SOCKS5 request from the stream. Validates VER=5, CMD=1 (CONNECT).
pub async fn read_request(stream: &mut TcpStream) -> AppResult<Socks5Request> {
    let mut hdr = [0u8; 4];
    stream
        .read_exact(&mut hdr)
        .await
        .map_err(|e| AppError::Validation(format!("socks5: request header read error: {e}")))?;

    if hdr[0] != 5 {
        return Err(AppError::Validation(format!(
            "socks5: bad version {}",
            hdr[0]
        )));
    }
    if hdr[1] != 1 {
        return Err(AppError::Validation(format!(
            "socks5: unsupported command {}",
            hdr[1]
        )));
    }

    let atyp = hdr[3];
    let addr = read_address(stream, atyp).await?;
    let mut port_buf = [0u8; 2];
    stream
        .read_exact(&mut port_buf)
        .await
        .map_err(|e| AppError::Validation(format!("socks5: port read error: {e}")))?;
    let port = u16::from_be_bytes(port_buf);

    Ok(Socks5Request { atyp, addr, port })
}

async fn read_address(stream: &mut TcpStream, atyp: u8) -> AppResult<String> {
    match atyp {
        1 => {
            // IPv4
            let mut ip = [0u8; 4];
            stream.read_exact(&mut ip).await?;
            let ipv4 = Ipv4Addr::from(ip);
            Ok(ipv4.to_string())
        }
        3 => {
            // Domain name
            let mut len_buf = [0u8; 1];
            stream.read_exact(&mut len_buf).await?;
            let len = len_buf[0] as usize;
            let mut domain = vec![0u8; len];
            stream.read_exact(&mut domain).await?;
            String::from_utf8(domain)
                .map_err(|_| AppError::Validation("socks5: invalid domain name".into()))
        }
        4 => {
            // IPv6
            let mut ip = [0u8; 16];
            stream.read_exact(&mut ip).await?;
            let ipv6 = std::net::Ipv6Addr::from(ip);
            Ok(ipv6.to_string())
        }
        _ => Err(AppError::Validation(format!(
            "socks5: unsupported ATYP {atyp}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn parse_ipv4_request() -> AppResult<()> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await?;
            let req = read_request(&mut s).await?;
            assert_eq!(req.atyp, 1);
            assert_eq!(req.addr, "93.184.216.34");
            assert_eq!(req.port, 443);
            AppResult::Ok(())
        });

        let mut client = TcpStream::connect(addr).await?;
        // VER=5, CMD=1, RSV=0, ATYP=1, IP=93.184.216.34, PORT=443
        client
            .write_all(&[5, 1, 0, 1, 93, 184, 216, 34, 1, 187])
            .await?;
        client.read_exact(&mut [0u8; 1]).await.ok(); // wait for close
        server.await.unwrap()?;
        Ok(())
    }

    #[tokio::test]
    async fn parse_domain_request() -> AppResult<()> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await?;
            let req = read_request(&mut s).await?;
            assert_eq!(req.atyp, 3);
            assert_eq!(req.addr, "example.com");
            assert_eq!(req.port, 80);
            AppResult::Ok(())
        });

        let mut client = TcpStream::connect(addr).await?;
        // VER=5, CMD=1, RSV=0, ATYP=3, LEN=11, "example.com", PORT=80
        let domain = b"example.com";
        let mut msg = vec![5, 1, 0, 3, domain.len() as u8];
        msg.extend_from_slice(domain);
        msg.extend_from_slice(&80u16.to_be_bytes());
        client.write_all(&msg).await?;
        client.read_exact(&mut [0u8; 1]).await.ok();
        server.await.unwrap()?;
        Ok(())
    }
}
