use crate::dns::{
    BytePacketReader, DNSDecodable, DNSEncodable, DNSName, DNSResult, ResourceClass, ResourceType,
};
use std::io::Write;

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: DNSName,
    pub type_: ResourceType, // TODO: Add RecordType Enum
    pub class: ResourceClass,
}

impl DNSEncodable for DNSQuestion {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DNSResult<()> {
        self.name.write_bytes(writer)?;
        let type_int: u16 = self.type_.into();
        writer
            .write_all(&type_int.to_be_bytes())
            .map_err(|e| e.to_string())?;
        let class_int: u16 = self.class.into();
        writer
            .write_all(&class_int.to_be_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl DNSDecodable for DNSQuestion {
    fn from_bytes(reader: &mut BytePacketReader) -> DNSResult<Self> {
        let name = DNSName::from_bytes(reader)?;
        let type_ = ResourceType::from(u16::from_be_bytes([reader.read()?, reader.read()?]));
        let class = ResourceClass::from(u16::from_be_bytes([reader.read()?, reader.read()?]));
        Ok(DNSQuestion { name, type_, class })
    }
}
