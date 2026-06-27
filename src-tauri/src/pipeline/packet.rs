use bytes::Bytes;

/// A data-plane packet — carries raw bytes through the processor pipeline.
///
/// `Clone` is O(1): `Bytes` is reference-counted.
#[derive(Debug, Clone)]
pub enum Packet {
    Tcp(Bytes),
    Udp(Bytes),
}

impl Packet {
    /// Payload length in bytes.
    pub fn len(&self) -> usize {
        match self {
            Packet::Tcp(b) | Packet::Udp(b) => b.len(),
        }
    }

    /// Whether the payload is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Read-only reference to the inner payload.
    pub fn payload(&self) -> &[u8] {
        match self {
            Packet::Tcp(b) | Packet::Udp(b) => b.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcp_packet_construction_and_len() {
        let p = Packet::Tcp(Bytes::from_static(b"hello"));
        assert_eq!(p.len(), 5);
        assert!(!p.is_empty());
        assert_eq!(p.payload(), b"hello");
    }

    #[test]
    fn empty_packet_is_empty() {
        let p = Packet::Tcp(Bytes::new());
        assert!(p.is_empty());
    }

    #[test]
    fn clone_is_shallow() {
        let p = Packet::Udp(Bytes::from_static(b"data"));
        let q = p.clone();
        assert_eq!(p.payload().as_ptr(), q.payload().as_ptr());
    }
}
