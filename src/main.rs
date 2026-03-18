use r_dns::dns::{
    BytePacketReader, DNSDecodable, DNSEncodable, DNSHeader, DNSName, DNSPacket, DNSQuestion
};
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create the Query packet
    let id = rand::random::<u16>();
    let mut query = DNSPacket::new(DNSHeader::new_query(id));
    query.questions.push(DNSQuestion { name: DNSName("www.google.com".to_string()), type_: 1, class: 1 });

    // "0.0.0.0:0" means:
    // 0.0.0.0 -> Listen on all my local network interfaces (WiFi, Ethernet).
    // :0      -> Let the OS pick any available random port for me.
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut query_buf: Vec<u8> = Vec::new();
    query.write_bytes(&mut query_buf)?;
    // Port 53 is the DNS port
    socket
        .send_to(&query_buf, "8.8.8.8:53")
        .expect("Failed to sent DNS packet");

    // DNS answer is usually less than 512 bytes
    let mut response_buf = [0u8; 512];

    match socket.recv(&mut response_buf) {
        Ok(_) => {
            let mut reader = BytePacketReader {
                buffer: response_buf,
                position: 0,
            };
            println!("Received response");
            let packet = DNSPacket::from_bytes(&mut reader)?;
            println!("{packet:#?}");
        }
        Err(err) => eprintln!("Encountered error while trying to receive response: {err}"),
    };

    Ok(())
}
