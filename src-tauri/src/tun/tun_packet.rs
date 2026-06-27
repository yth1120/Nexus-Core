use bytes::Bytes;

use crate::pipeline::Packet;
use crate::utils::AppResult;

#[derive(Debug, Clone)]
pub enum TunPacket {
    IPv4(Bytes),
    IPv6(Bytes),
}

impl TunPacket {
    pub fn len(&self) -> usize {
        match self {
            TunPacket::IPv4(b) | TunPacket::IPv6(b) => b.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert a pipeline Packet into a TunPacket (raw IP payload extraction).
    pub fn from_packet(p: &Packet) -> AppResult<Self> {
        let data = p.payload();
        if data.is_empty() {
            return Ok(TunPacket::IPv4(Bytes::new()));
        }
        match data[0] >> 4 {
            4 => Ok(TunPacket::IPv4(Bytes::copy_from_slice(data))),
            6 => Ok(TunPacket::IPv6(Bytes::copy_from_slice(data))),
            _ => Ok(TunPacket::IPv4(Bytes::copy_from_slice(data))),
        }
    }

    /// Convert into a pipeline Packet (raw bytes wrapping).
    pub fn into_packet(self) -> Packet {
        match self {
            TunPacket::IPv4(b) | TunPacket::IPv6(b) => Packet::Tcp(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ipv4_roundtrip() -> AppResult<()> {
        let data = vec![
            0x45u8, 0, 0, 20, 0, 0, 0, 0, 64, 0, 0, 0, 127, 0, 0, 1, 127, 0, 0, 2,
        ];
        let p = Packet::Tcp(Bytes::from(data));
        let tp = TunPacket::from_packet(&p)?;
        assert!(matches!(tp, TunPacket::IPv4(_)));
        let back = tp.into_packet();
        assert_eq!(back.len(), 20);
        Ok(())
    }
}
