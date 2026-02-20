use std::io::{Result, Write};

pub mod header;
pub mod question;
pub mod types;

// Declare explicitly, for more convenient use.
pub use header::DNSHeader;
pub use question::DNSQuestion;
pub use types::{DNSEncodable, DnsName};

pub struct DNSPacket {
    pub header: DNSHeader,
    pub question: DNSQuestion,
}

impl DNSEncodable for DNSPacket {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.header.write_bytes(writer)?;
        self.question.write_bytes(writer)?;
        Ok(())
    }
}
