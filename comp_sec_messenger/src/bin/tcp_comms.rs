use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;


fn main() -> std::io::Result<()> {

// // instead of using a DNS server, scan the local network for IPs accepting a TCP connection
// let address = [0, 0, 0, 0];
// const CIDR: i64 = 0; // generally follows an ip, e. g. 127.0.0.1/24 denotes 127.0.0.1 with a subnet mask of the first 24 bits (255.255.255.0)

// let mut addrs: [SocketAddr; 2 << (32 - CIDR)] = [SocketAddr::from(([127, 0, 0, 1], 8080)); 2 << (32 - CIDR)];
// let address_int = (address[0] << 24) + (address[1] << 16) + (address[2] << 8) + address[1];
// let mask_int: i128 = ((2 << CIDR - 1) - 1) << 8;

// // enumerate all possible addresses...
// for i in (1..addrs.len()).step_by(2){
//     let mut base_addr : i128 = (address_int & mask_int) + ((i / 2) as i128);
//     let new_addr = [
//         ((base_addr & 4278190080 ) >> 24) as u8,
//         ((base_addr & 16711680) >> 16) as u8,
//         ((base_addr & 65280) >> 8) as u8,
//         (base_addr & 255) as u8
//     ];

//     addrs[i] = SocketAddr::from((new_addr, 8081));
//     addrs[i - 1] = SocketAddr::from((new_addr, 8080));
// }

let addrs = [SocketAddr::from(([0,0,0,0], 8081)), SocketAddr::from(([0,0,0,0], 8080))];


let mut connected = false;
for address in addrs {
    if let Ok(mut stream) = TcpStream::connect_timeout(&address, Duration::new(0, 1)){
        println!("Connected to the server!");
        stream.write(&[1,2,3,4])?;
        connected = true;
        break;
    }
}
    if !connected {

    println!("Opposite Party is not logged on, creating TCP Listener");
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        message_received(stream?);
    }
    };

    Ok(())

}


// if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
//     
// } else {
//     println!("Opposite Party is not logged on, creating TCP Listener");
//     let listener = TcpListener::bind("129.15.65.229:8080")?;

//     // accept connections and process them serially
//     for stream in listener.incoming() {
//         message_received(stream?);
//     }
//     Ok(())
// }

fn message_received(mut stream: TcpStream) -> std::io::Result<()>{
    let buf = &mut [0; 128];
    stream.read(buf)?;
    println!("{:?}", buf);
    Ok(())
}
