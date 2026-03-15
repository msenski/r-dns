use std::io::Write;

pub mod header;
pub mod name;
pub mod question;
pub mod types;

// Declare explicitly, for more convenient use.
pub use header::DNSHeader;
pub use name::DnsName;
pub use question::DNSQuestion;
pub use types::{BytePacketReader, DNSDecodable, DNSEncodable, DnsResult};

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

#[derive(Debug)]
pub struct DNSRecord {
    pub name: DnsName,
    pub type_: u16,
    pub class: u16,
    pub ttl: u32,
    pub data_len: u16,
    pub data: Vec<u8>,
}

impl DNSDecodable for DNSRecord {
    fn from_bytes(reader: &mut BytePacketReader) -> DnsResult<Self> {
        let name = DnsName::from_bytes(reader)?;
        let type_ = u16::from_be_bytes([reader.read()?, reader.read()?]);
        let class = u16::from_be_bytes([reader.read()?, reader.read()?]);
        let ttl = u32::from_be_bytes([
            reader.read()?,
            reader.read()?,
            reader.read()?,
            reader.read()?,
        ]);
        let data_len = u16::from_be_bytes([reader.read()?, reader.read()?]);
        let mut data = Vec::new();
        for _ in 0..data_len {
            data.push(reader.read()?);
        }
        return Ok(DNSRecord {
            name,
            type_,
            class,
            ttl,
            data_len,
            data,
        });
    }
}
