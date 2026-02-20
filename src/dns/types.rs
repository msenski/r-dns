/// Common types
use std::io::{Result, Write};

// TODO: Create a BigEndianWriter wrapper to enforce endianness at the type level.

/// A trait for types that can be serialized into the DNS wire format.
///
/// # Requirements
/// Implementations of this trait are responsible for ensuring that all
/// multi-byte numerical fields are written in Big-Endian (Network Byte Order)
/// as per RFC 1035.
pub trait DNSEncodable {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<()>;
}

#[derive(Debug)]
pub struct DnsName(pub String);

impl DNSEncodable for DnsName {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        for part in self.0.split(".") {
            writer.write_all(&[part.len() as u8])?;
            writer.write_all(part.as_bytes())?;
        }
        writer.write_all(&[0])?;
        Ok(())
    }
}
