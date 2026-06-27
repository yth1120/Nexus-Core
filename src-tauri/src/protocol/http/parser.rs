use crate::utils::{AppError, AppResult};

use super::request::ProxyRequest;

/// Parse an HTTP CONNECT request from raw bytes.
///
/// Only handles CONNECT — returns `AppError::Validation` for any other method
/// or malformed input. No full HTTP framework is implemented.
pub fn parse_connect_request(buf: &[u8]) -> AppResult<ProxyRequest> {
    let text = std::str::from_utf8(buf)
        .map_err(|_| AppError::Validation("http: invalid UTF-8 in request".into()))?;

    let mut lines = text.lines();
    let first_line = lines
        .next()
        .ok_or_else(|| AppError::Validation("http: empty request".into()))?;

    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(AppError::Validation(format!(
            "http: malformed request line: {first_line}"
        )));
    }

    let method = parts[0];
    if !method.eq_ignore_ascii_case("CONNECT") {
        return Err(AppError::Validation(format!(
            "http: unsupported method: {method}"
        )));
    }

    let target = parts[1];
    let (host, port_str) = target
        .rsplit_once(':')
        .ok_or_else(|| AppError::Validation(format!("http: missing port in {target}")))?;

    let host = host.trim();
    if host.is_empty() {
        return Err(AppError::Validation("http: empty host".into()));
    }

    let port: u16 = port_str
        .parse()
        .map_err(|_| AppError::Validation(format!("http: invalid port: {port_str}")))?;

    let _headers: Vec<String> = lines
        .take_while(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    Ok(ProxyRequest::new(method, host, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_connect() -> AppResult<()> {
        let req = parse_connect_request(
            b"CONNECT example.com:443 HTTP/1.1\r\nHost: example.com\r\n\r\n",
        )?;
        assert_eq!(req.method, "CONNECT");
        assert_eq!(req.host, "example.com");
        assert_eq!(req.port, 443);
        assert_eq!(req.headers.len(), 1);
        Ok(())
    }

    #[test]
    fn missing_port_is_error() {
        assert!(parse_connect_request(b"CONNECT example.com HTTP/1.1\r\n\r\n").is_err());
    }

    #[test]
    fn non_connect_method_is_error() {
        assert!(parse_connect_request(b"GET / HTTP/1.1\r\n\r\n").is_err());
    }

    #[test]
    fn empty_host_is_error() {
        assert!(parse_connect_request(b"CONNECT :443 HTTP/1.1\r\n\r\n").is_err());
    }
}
