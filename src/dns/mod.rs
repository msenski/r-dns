use std::io::Write;

pub mod header;
pub mod name;
pub mod question;
pub mod types;

// Declare explicitly, for more convenient use.
pub use header::DNSHeader;
pub use name::DnsName;
pub use question::DNSQuestion;
pub use types::{DnsResult, BytePacketReader, DNSEncodable, DNSDecodable};

pub struct DNSPacket {
    pub header: DNSHeader,
    pub question: DNSQuestion,
}

impl DNSEncodable for DNSPacket {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DnsResult<()> {
        self.header.write_bytes(writer)?;
        self.question.write_bytes(writer)?;
        Ok(())
    }
}
