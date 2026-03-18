use crate::dns::{BytePacketReader, DNSDecodable, DNSEncodable, DNSName, DNSResult};
use std::io::Write;

#[derive(Debug)]
pub struct DNSRecord {
    pub name: DNSName,
    pub type_: u16,
    pub class: u16,
    pub ttl: u32,
    // omit adding a `data_length`` property, as it can be derived
    // from `self.data` and should not be se manually
    pub data: Vec<u8>,
}

impl DNSEncodable for DNSRecord {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DNSResult<()> {
        self.name.write_bytes(writer)?;
        writer
            .write_all(&self.type_.to_be_bytes())
            .map_err(|e| e.to_string())?;
        writer
            .write_all(&self.class.to_be_bytes())
            .map_err(|e| e.to_string())?;
        writer
            .write_all(&self.ttl.to_be_bytes())
            .map_err(|e| e.to_string())?;
        let data_len = self.data.len();
        writer
            .write_all(&data_len.to_be_bytes())
            .map_err(|e| e.to_string())?;
        writer.write_all(&self.data).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl DNSDecodable for DNSRecord {
    fn from_bytes(reader: &mut BytePacketReader) -> DNSResult<Self> {
        let name = DNSName::from_bytes(reader)?;
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
            data,
        });
    }
}
