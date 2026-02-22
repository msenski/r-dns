use crate::dns::{
    DNSEncodable,
    types::{BytePacketReader, DNSDecodable, DnsResult},
};
use std::io::Write;

pub const RECURSION_DESIRED: u16 = 1 << 8;

#[derive(Debug)]
pub struct DNSHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl DNSEncodable for DNSHeader {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DnsResult<()> {
        writer.write_all(&self.id.to_be_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(&self.flags.to_be_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(&self.num_questions.to_be_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(&self.num_answers.to_be_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(&self.num_authorities.to_be_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(&self.num_additionals.to_be_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl DNSDecodable for DNSHeader {
    fn from_bytes(reader: &mut BytePacketReader) -> DnsResult<Self> {
        Ok(DNSHeader {
            id: u16::from_be_bytes([reader.read()?, reader.read()?]),
            flags: u16::from_be_bytes([reader.read()?, reader.read()?]),
            num_questions: u16::from_be_bytes([reader.read()?, reader.read()?]),
            num_answers: u16::from_be_bytes([reader.read()?, reader.read()?]),
            num_authorities: u16::from_be_bytes([reader.read()?, reader.read()?]),
            num_additionals: u16::from_be_bytes([reader.read()?, reader.read()?]),
        })
    }
}

impl DNSHeader {
    fn new(id: u16) -> Self {
        DNSHeader {
            id,
            flags: 0,
            num_questions: 0,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        }
    }

    pub fn new_query(id: u16) -> Self {
        let mut header = Self::new(id);
        header.flags = RECURSION_DESIRED;
        header.num_questions = 1;
        header
    }
}
