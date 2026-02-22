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
    fn from_bytes(buffer: &mut BytePacketReader) -> DnsResult<Self>
    where
        Self: Sized;
}

/// Holds the parsed DNS name, like "example.com".
#[derive(Debug)]
pub struct DnsName(pub String);

impl DNSEncodable for DnsName {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DnsResult<()> {
        for part in self.0.split(".") {
            // In DNS, the header alternates between a length byte
            //(indicating the length of the following string
            // and the actual text bytes (excl. dots). A "0"-length byte
            // represents the end. Example: [7]example[3]com[0]
            writer.write_all(&[part.len() as u8]).map_err(|e| e.to_string())?; // length-byte
            writer.write_all(part.as_bytes()).map_err(|e| e.to_string())?; // actual string
        }
        writer.write_all(&[0]).map_err(|e| e.to_string())?;
        Ok(())
    }
}
