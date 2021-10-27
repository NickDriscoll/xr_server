use std::net::{UdpSocket, TcpListener};
use std::time::Duration;



fn main() {
    println!("Starting server");

    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();
    for stream in listener.incoming() {

    }        
}
