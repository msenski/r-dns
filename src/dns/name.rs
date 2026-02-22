use std::io::Write;

use crate::dns::{DNSEncodable, DNSDecodable, BytePacketReader,DnsResult};

/// Number 192 in hex (11000000 in binary). If the first 2 bits 
/// a length segment are 11, they indicate a compression. Compression
/// is used in DNS to point to the DNS name
const COMPRESSION_PRREFIX: u8 = 0xC; 

/// Holds the parsed DNS name, like "example.com".
#[derive(Debug)]
pub struct DnsName(pub String);

impl DNSEncodable for DnsName {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DnsResult<()> {
        for part in self.0.split(".") {
            // In DNS, the header alternates between a length byte
            //(indicating the length of the following string
            // and the actual text bytes (excl. dots). A "0"-length byte
            // represents the end. Example: [7]example[3]com[0]
            writer.write_all(&[part.len() as u8]).map_err(|e| e.to_string())?; // length-byte
            writer.write_all(part.as_bytes()).map_err(|e| e.to_string())?; // actual string
        }
        writer.write_all(&[0]).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl DNSDecodable for DnsName {
    fn from_bytes(reader: &mut BytePacketReader) -> DnsResult<Self>{
        let mut name = String::new();
        let mut add_dot = false;

        while let Ok(len) = reader.read() {
            if len == 0u8 {
                // A 0-byte indicates the end of the name
                break;
            }
            // Bitwise AND - filter out the two front bits and chck if they are set
            else if len & COMPRESSION_PRREFIX == COMPRESSION_PRREFIX {
                unimplemented!("The compression-pointer mechanism is not implemented yet")
            }
            if add_dot{
                name.push('.');
            }
            for _ in 0..len {
                name.push(reader.read()? as char);
                add_dot = true;
            }
        }
        Ok(DnsName(name))
    }
}