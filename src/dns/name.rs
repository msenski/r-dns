use std::io::Write;

use crate::dns::{BytePacketReader, DNSDecodable, DNSEncodable, DnsResult};

/// Number 192 in hex (11000000 in binary). If the first 2 bits
/// a length segment are 11, they indicate a compression. Compression
/// is used in DNS to point to the DNS name
const COMPRESSION_PRREFIX: u8 = 0xC0;

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
            writer
                .write_all(&[part.len() as u8])
                .map_err(|e| e.to_string())?; // length-byte
            writer
                .write_all(part.as_bytes())
                .map_err(|e| e.to_string())?; // actual string
        }
        writer.write_all(&[0]).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl DNSDecodable for DnsName {
    fn from_bytes(reader: &mut BytePacketReader) -> DnsResult<Self> {
        let mut name = String::new();
        let mut add_dot = false;

        let mut pos = reader.position;

        // Indicates if a jump to a pointer happened
        let mut jumped = false;
        let mut jump_pos = 0;

        loop {
            // Read-in the length byte
            let len = reader.get(pos)?;

            // To save space DNS uses compression (see RFC 1035, section 4.1.4). See,
            // if we are dealing with a compression sequence first.
            if len & COMPRESSION_PRREFIX == COMPRESSION_PRREFIX {
                // According to the RFC, "The pointer takes the form of a two octet sequence" So the
                // form is like "11xxxxxx yyyyyyyy" (x: bit from 1st byte, y: bit from 2nd byte).
                let mut b1: u16 = len as u16;

                // First, ignore tha compression bits by flipping them with XOR:
                b1 ^= COMPRESSION_PRREFIX as u16;
                // Shift the bits of the 1st byte to the high end
                b1 <<= 8;
                // Get the 2nd byte of the pointer
                let b2: u16 = reader.get(pos + 1)? as u16;
                // Finally, combine both bytes into final offset
                let offset = b1 | b2;

                if !jumped {
                    // If this is th first time we encounter a pointer, remember the position to
                    // jump back to after we are done with compression handling. Add 2 since the
                    // above pointer occupies two octets (2 bytes).
                    jump_pos = pos + 2;
                    jumped = true;
                }
                pos = offset as usize;

            // If it is not a compression sequence, read-in the name as usual.
            } else {
                if len == 0u8 {
                    // A 0-byte indicates the end of the name. Move past the length
                    // byte and break out of the loop.
                    pos += 1;
                    break;
                } else {
                    // Move position forward, past the length byte
                    pos += 1;

                    if add_dot {
                        name.push('.');
                    }
                    for _ in 0..len {
                        name.push(reader.get(pos)? as char);
                        pos += 1;
                    }
                    add_dot = true;
                }
            }
        }

        if jumped {
            reader.position = jump_pos;
        } else {
            reader.position = pos;
        }

        Ok(DnsName(name))
    }
}
