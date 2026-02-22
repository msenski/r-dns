use std::io::Write;

pub type DnsResult<T> = std::result::Result<T, String>;

pub struct BytePacketReader {
    pub buffer: [u8; 512], // DNS protocol (RFC 1035) limits UDP messages to 512 bytes.
    pub position: usize,
}

impl BytePacketReader {
    /// Reads one byte, and moves the `self.position` forward.
    pub fn read(&mut self) -> DnsResult<u8> {
        if self.position >= self.buffer.len() {
            return Err("End of buffer reached.".to_string());
        }
        let res = self.buffer[self.position];
        self.position += 1;
        Ok(res)
    }

    pub fn get(&self, pos: usize) -> DnsResult<u8> {
        if pos >= self.buffer.len() {
            return Err(format!("End of buffer at position {}", pos));
        }
        Ok(self.buffer[pos])
    }
}

// TODO: Create a BigEndianWriter wrapper to enforce endianness at the type level.

/// A trait for types that can be serialized into the DNS wire format.
///
/// # Requirements
/// Implementations of this trait are responsible for ensuring that all
/// multi-byte numerical fields are written in Big-Endian (Network Byte Order)
/// as per RFC 1035.
pub trait DNSEncodable {
    /// Encodes `Self` into bytes and writes them into the `writer`.
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DnsResult<()>;
}

/// A trait for types that can be de-serialized from the DNS wire format.
pub trait DNSDecodable {
    fn from_bytes(reader: &mut BytePacketReader) -> DnsResult<Self>
    where
        Self: Sized;
}

