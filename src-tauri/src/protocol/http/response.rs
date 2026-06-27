/// A minimal HTTP response for proxy use.
#[derive(Debug, Clone)]
pub struct ProxyResponse {
    pub status: u16,
    pub reason: String,
}

impl ProxyResponse {
    pub fn ok() -> Self {
        Self {
            status: 200,
            reason: "Connection Established".into(),
        }
    }

    pub fn bad_request() -> Self {
        Self {
            status: 400,
            reason: "Bad Request".into(),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            status: 500,
            reason: "Internal Server Error".into(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!("HTTP/1.1 {} {}\r\n\r\n", self.status, self.reason).into_bytes()
    }
}
