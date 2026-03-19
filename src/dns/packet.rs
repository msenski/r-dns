use crate::dns::{
    BytePacketReader, DNSDecodable, DNSEncodable, DNSHeader, DNSQuestion, DNSRecord, DNSResult,
};
use std::io::Write;

#[derive(Debug)]
pub struct DNSPacket {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSRecord>,
    pub authorities: Vec<DNSRecord>,
    pub additionals: Vec<DNSRecord>,
}

impl DNSPacket {
    /// A helper to create a clean packet with empty lists
    pub fn new(header: DNSHeader) -> Self {
        DNSPacket {
            header,
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }
}

impl DNSEncodable for DNSPacket {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DNSResult<()> {
        self.header.write_bytes(writer)?;
        for question in &self.questions {
            question.write_bytes(writer)?;
        }
        for answer in &self.answers {
            answer.write_bytes(writer)?;
        }
        for authority in &self.authorities {
            authority.write_bytes(writer)?;
        }
        for additional in &self.additionals {
            additional.write_bytes(writer)?;
        }
        Ok(())
    }
}

impl DNSDecodable for DNSPacket {
    fn from_bytes(reader: &mut BytePacketReader) -> DNSResult<Self>
    where
        Self: Sized,
    {
        let header = DNSHeader::from_bytes(reader)?;
        let mut questions = Vec::new();
        println!("Parsing Header:");
        println!("{header:#?}");
        for _ in 0..header.num_questions {
            questions.push(DNSQuestion::from_bytes(reader)?);
        }
        let mut answers = Vec::new();
        for _ in 0..header.num_answers {
            answers.push(DNSRecord::from_bytes(reader)?);
        }
        let mut authorities = Vec::new();
        for _ in 0..header.num_authorities {
            authorities.push(DNSRecord::from_bytes(reader)?);
        }
        let mut additionals = Vec::new();
        for _ in 0..header.num_additionals {
            additionals.push(DNSRecord::from_bytes(reader)?);
        }
        return Ok(DNSPacket {
            header,
            questions,
            answers,
            authorities,
            additionals,
        });
    }
}
