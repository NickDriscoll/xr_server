use std::net::{UdpSocket};
use std::time::Duration;

fn main() {
    println!("Starting server");
    let listener = UdpSocket::bind("0.0.0.0:6969").unwrap();
    
    let mut packet_buffer = [0u8; 16];
    while let Ok((size, socket_addr)) = listener.recv_from(&mut packet_buffer) {
        println!("Received {} bytes\n{:?}", size, packet_buffer);        
    }
}
