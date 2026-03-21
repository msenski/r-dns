use crate::dns::{
    BytePacketReader, DNSDecodable, DNSEncodable, DNSName, DNSResult, Data, RecordClass, RecordType,
};
use std::{io::Write, net::Ipv4Addr};

#[derive(Debug)]
pub struct DNSRecord {
    pub name: DNSName,
    pub type_: RecordType,
    pub class: RecordClass,
    pub ttl: u32,
    // omit adding a `data_length`` property, as it can be derived
    // from `self.data` and should not be se manually
    pub data: Data,
}

impl DNSEncodable for DNSRecord {
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

        writer
            .write_all(&self.ttl.to_be_bytes())
            .map_err(|e| e.to_string())?;

        let data_len = self.data.len();
        writer
            .write_all(&data_len.to_be_bytes())
            .map_err(|e| e.to_string())?;

        self.data.write_bytes(writer)?;
        Ok(())
    }
}

impl DNSDecodable for DNSRecord {
    fn from_bytes(reader: &mut BytePacketReader) -> DNSResult<Self> {
        let name = DNSName::from_bytes(reader)?;
        let type_ = RecordType::from(u16::from_be_bytes([reader.read()?, reader.read()?]));
        let class = RecordClass::from(u16::from_be_bytes([reader.read()?, reader.read()?]));
        let ttl = u32::from_be_bytes([
            reader.read()?,
            reader.read()?,
            reader.read()?,
            reader.read()?,
        ]);
        let data_len = u16::from_be_bytes([reader.read()?, reader.read()?]);
        let mut data_bytes = vec![0u8; data_len as usize];
        for i in 0..data_len {
            data_bytes[i as usize] = reader.read()?;
        }
        let data = match type_ {
            RecordType::A => {
                let octets: [u8; 4] = data_bytes
                    .try_into()
                    .map_err(|_| "Ipv4 address must be exactly 4 bytes".to_string())?;
                Data::IPv4(Ipv4Addr::from_octets(octets))
            }
            _ => Data::Unknown(data_bytes),
        };
        return Ok(DNSRecord {
            name,
            type_,
            class,
            ttl,
            data,
        });
    }
}
