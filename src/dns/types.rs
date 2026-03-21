use std::io::Write;
use std::net::Ipv4Addr;

pub type DNSResult<T> = std::result::Result<T, String>;

pub struct BytePacketReader {
    pub buffer: [u8; 512], // DNS protocol (RFC 1035) limits UDP messages to 512 bytes.
    pub position: usize,
}

impl BytePacketReader {
    /// Reads one byte, and moves the `self.position` forward.
    pub fn read(&mut self) -> DNSResult<u8> {
        if self.position >= self.buffer.len() {
            return Err("End of buffer reached.".to_string());
        }
        let res = self.buffer[self.position];
        self.position += 1;
        Ok(res)
    }

    /// Returns the byte at position `pos`, if present.
    pub fn get(&self, pos: usize) -> DNSResult<u8> {
        if pos >= self.buffer.len() {
            return Err(format!("End of buffer at position {}", pos));
        }
        Ok(self.buffer[pos])
    }
}

// TODO: Create a BigEndianWriter wrapper to enforce endianness at the type level.

/// A trait for types that can be serialized into the DNS wire format.
///
/// # Requirements
/// Implementations of this trait are responsible for ensuring that all
/// multi-byte numerical fields are written in Big-Endian (Network Byte Order)
/// as per RFC 1035.
pub trait DNSEncodable {
    /// Encodes `Self` into bytes and writes them into the `writer`.
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DNSResult<()>;
}

/// A trait for types that can be de-serialized from the DNS wire format.
pub trait DNSDecodable {
    fn from_bytes(reader: &mut BytePacketReader) -> DNSResult<Self>
    where
        Self: Sized;
}

/// Represents the CLASS of the DNS resource data. See RFC 1035,
/// section 3.2.4 for details.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum RecordClass {
    IN = 1, // Internet
    CS = 2, // CSNET
    CH = 3, // CHAOS
    HS = 4, // Hesiod
    UNKNOWN(u16),
}

impl From<u16> for RecordClass {
    fn from(value: u16) -> Self {
        match value {
            1 => RecordClass::IN,
            2 => RecordClass::CS,
            3 => RecordClass::CH,
            4 => RecordClass::HS,
            _ => RecordClass::UNKNOWN(value),
        }
    }
}

impl From<RecordClass> for u16 {
    fn from(variant: RecordClass) -> Self {
        match variant {
            RecordClass::IN => 1,
            RecordClass::CS => 2,
            RecordClass::CH => 3,
            RecordClass::HS => 4,
            RecordClass::UNKNOWN(val) => val,
        }
    }
}

/// Represents the TYPE of the DNS resource data. See RFC 1035,
/// section 3.2.4 for details.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum RecordType {
    A = 1,        // IPv4 Address
    NS = 2,       // Name Server
    CNAME = 5,    // Canonical Name (Alias)
    SOA = 6,      // Start of Authority
    WKS = 11,     // Well Known Service
    PTR = 12,     // Domain Name Pointer
    HINFO = 13,   // Host Information
    MINFO = 14,   // Mailbox/Mail List Information
    MX = 15,      // Mail Exchange
    TXT = 16,     // Text Strings
    AAAA = 28,    // IPv6 Address
    SRV = 33,     // Service Locator
    UNKNOWN(u16), // Catch-all for anything unexpected
}

impl From<u16> for RecordType {
    fn from(value: u16) -> Self {
        match value {
            1 => RecordType::A,
            2 => RecordType::NS,
            5 => RecordType::CNAME,
            6 => RecordType::SOA,
            11 => RecordType::WKS,
            12 => RecordType::PTR,
            13 => RecordType::HINFO,
            14 => RecordType::MINFO,
            15 => RecordType::MX,
            16 => RecordType::TXT,
            28 => RecordType::AAAA,
            33 => RecordType::SRV,
            _ => RecordType::UNKNOWN(value),
        }
    }
}

impl From<RecordType> for u16 {
    fn from(variant: RecordType) -> Self {
        match variant {
            RecordType::A => 1,
            RecordType::NS => 2,
            RecordType::CNAME => 5,
            RecordType::SOA => 6,
            RecordType::WKS => 11,
            RecordType::PTR => 12,
            RecordType::HINFO => 13,
            RecordType::MINFO => 14,
            RecordType::MX => 15,
            RecordType::TXT => 16,
            RecordType::AAAA => 28,
            RecordType::SRV => 33,
            RecordType::UNKNOWN(value) => value,
        }
    }
}

#[derive(Debug)]
pub enum Data {
    IPv4(Ipv4Addr),
    Unknown(Vec<u8>), // TODO: Add other data types
}

impl DNSEncodable for Data {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> DNSResult<()> {
        match self {
            Data::IPv4(addr) => writer.write_all(&addr.octets()).map_err(|e| e.to_string()),
            Data::Unknown(data) => writer.write_all(&data).map_err(|e| e.to_string()),
        }
    }
}

impl Data {
    pub fn len(&self) -> u16 {
        match self {
            Data::IPv4(_) => 4,
            Data::Unknown(data) => data.len() as u16,
        }
    }
}
