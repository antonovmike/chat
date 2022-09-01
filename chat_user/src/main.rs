use std::net::TcpStream;

mod structures;
mod transmitter;

const LOCAL: &str = "127.0.0.1:6000";
// const STRUCT_SIZE: usize = 96;

fn main() {
    let client = TcpStream::connect(LOCAL)
        .expect("Stream failed to connect");
    client.set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
        transmitter::transmitter(client);
}