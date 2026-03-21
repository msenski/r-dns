use r_dns::dns::{
    BytePacketReader, DNSDecodable, DNSEncodable, DNSHeader, DNSName, DNSPacket, DNSQuestion,
    DNSResult, RecordClass, RecordType,
};
use std::net::{Ipv4Addr, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = Ipv4Addr::new(8, 8, 8, 8);
    let domain_name = DNSName("www.google.com".to_string());
    let record_type = RecordType::A;

    let packet = send_query(ip, domain_name, record_type)?;
    println!("{packet:#?}");
    Ok(())
}

// def send_query(ip_address, domain_name, record_type):
// query = build_query(domain_name, record_type)
// sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
// sock.sendto(query, (ip_address, 53))

// data, _ = sock.recvfrom(1024)
// return parse_dns_packet(data)

fn send_query(ip: Ipv4Addr, domain_name: DNSName, record_type: RecordType) -> DNSResult<DNSPacket> {
    // create the query packet
    let id = rand::random::<u16>();
    let mut query = DNSPacket::new(DNSHeader::new_query(id));
    query.questions.push(DNSQuestion {
        name: domain_name,
        type_: record_type,
        class: RecordClass::IN,
    });

    let mut query_buf: Vec<u8> = Vec::new();
    query.write_bytes(&mut query_buf)?;

    // "0.0.0.0:0" means:
    // 0.0.0.0 -> Listen on all my local network interfaces (WiFi, Ethernet).
    // :0      -> Let the OS pick any available random port for me.
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("Could not open socket: {e}"))?;

    // Connect to the target IP address, such that we only
    // send/receive packets to/from that address.
    // Port 53 is the DNS port
    socket.connect((ip, 53)).map_err(|e|e.to_string())?;

    socket.send(&query_buf).expect("Failed to sent DNS packet");

    // DNSPacket is usually less than 512 bytes
    let mut response_buf = [0u8; 512];

    match socket.recv(&mut response_buf) {
        Ok(_) => {
            let mut reader = BytePacketReader {
                buffer: response_buf,
                position: 0,
            };
            return DNSPacket::from_bytes(&mut reader);
        }
        Err(e) => Err(format!(
            "Encountered error while trying to receive response: {e}"
        )),
    }
}
