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
pub enum ResourceClass {
    IN = 1, // Internet
    CS = 2, // CSNET
    CH = 3, // CHAOS
    HS = 4, // Hesiod
    UNKNOWN(u16),
}

impl From<u16> for ResourceClass {
    fn from(value: u16) -> Self {
        match value {
            1 => ResourceClass::IN,
            2 => ResourceClass::CS,
            3 => ResourceClass::CH,
            4 => ResourceClass::HS,
            _ => ResourceClass::UNKNOWN(value),
        }
    }
}

impl From<ResourceClass> for u16 {
    fn from(variant: ResourceClass) -> Self {
        match variant {
            ResourceClass::IN => 1,
            ResourceClass::CS => 2,
            ResourceClass::CH => 3,
            ResourceClass::HS => 4,
            ResourceClass::UNKNOWN(val) => val,
        }
    }
}

/// Represents the TYPE of the DNS resource data. See RFC 1035,
/// section 3.2.4 for details.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum ResourceType {
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

impl From<u16> for ResourceType {
    fn from(value: u16) -> Self {
        match value {
            1 => ResourceType::A,
            2 => ResourceType::NS,
            5 => ResourceType::CNAME,
            6 => ResourceType::SOA,
            11 => ResourceType::WKS,
            12 => ResourceType::PTR,
            13 => ResourceType::HINFO,
            14 => ResourceType::MINFO,
            15 => ResourceType::MX,
            16 => ResourceType::TXT,
            28 => ResourceType::AAAA,
            33 => ResourceType::SRV,
            _ => ResourceType::UNKNOWN(value),
        }
    }
}

impl From<ResourceType> for u16 {
    fn from(variant: ResourceType) -> Self {
        match variant {
            ResourceType::A => 1,
            ResourceType::NS => 2,
            ResourceType::CNAME => 5,
            ResourceType::SOA => 6,
            ResourceType::WKS => 11,
            ResourceType::PTR => 12,
            ResourceType::HINFO => 13,
            ResourceType::MINFO => 14,
            ResourceType::MX => 15,
            ResourceType::TXT => 16,
            ResourceType::AAAA => 28,
            ResourceType::SRV => 33,
            ResourceType::UNKNOWN(value) => value,
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
