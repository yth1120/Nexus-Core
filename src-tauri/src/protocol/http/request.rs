/// A parsed HTTP CONNECT request — the minimum needed for proxy operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyRequest {
    pub method: String,
    pub host: String,
    pub port: u16,
    pub headers: Vec<String>,
}

impl ProxyRequest {
    pub fn new(method: &str, host: &str, port: u16) -> Self {
        Self {
            method: method.to_string(),
            host: host.to_string(),
            port,
            headers: Vec::new(),
        }
    }
}
