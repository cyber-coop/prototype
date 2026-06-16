use super::error::DeserializeError;
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use varint::VarInt;

pub struct Addr {
    pub addrs: Vec<SocketAddr>,
}

impl Addr {
    pub fn deserialize(bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let count = VarInt::decode(cur.split().1)?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut addrs = Vec::new();

        for _ in 0..count {
            // timestamp (4 bytes, protocol version >= 31402)
            let mut buf = [0u8; 4];
            cur.read_exact(&mut buf)?;

            // services (8 bytes)
            let mut buf = [0u8; 8];
            cur.read_exact(&mut buf)?;

            // IP (16 bytes, IPv4-mapped IPv6: ::ffff:a.b.c.d)
            let mut ip_buf = [0u8; 16];
            cur.read_exact(&mut ip_buf)?;

            // port (2 bytes, big-endian / network byte order)
            let mut port_buf = [0u8; 2];
            cur.read_exact(&mut port_buf)?;
            let port = u16::from_be_bytes(port_buf);

            let ip = if ip_buf[..12] == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff] {
                IpAddr::V4(Ipv4Addr::new(ip_buf[12], ip_buf[13], ip_buf[14], ip_buf[15]))
            } else {
                IpAddr::V6(Ipv6Addr::from(ip_buf))
            };

            addrs.push(SocketAddr::new(ip, port));
        }

        Ok(Self { addrs })
    }
}
