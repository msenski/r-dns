use crate::dns::{BytePacketReader, DNSDecodable, DNSEncodable, DNSName, DNSResult};
use std::io::Write;

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: DNSName,
    pub type_: u16, // TODO: Add RecordType Enum
    pub class: u16,
}

impl DNSEncodable for DNSQuestion {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DNSResult<()> {
        self.name.write_bytes(writer)?;
        writer
            .write_all(&self.type_.to_be_bytes())
            .map_err(|e| e.to_string())?;
        writer
            .write_all(&self.class.to_be_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl DNSDecodable for DNSQuestion {
    fn from_bytes(reader: &mut BytePacketReader) -> DNSResult<Self> {
        let name = DNSName::from_bytes(reader)?;
        let type_ = u16::from_be_bytes([reader.read()?, reader.read()?]);
        let class = u16::from_be_bytes([reader.read()?, reader.read()?]);
        Ok(DNSQuestion { name, type_, class })
    }
}
