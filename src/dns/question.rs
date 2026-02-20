use crate::dns::{DNSEncodable, DnsName};
use std::io::{Result, Write};

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: DnsName,
    pub type_: u16, // TODO: Add RecordType Enum
    pub class: u16,
}

impl DNSEncodable for DNSQuestion {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.name.write_bytes(writer)?;
        writer.write_all(&self.type_.to_be_bytes())?;
        writer.write_all(&self.class.to_be_bytes())?;
        Ok(())
    }
}
