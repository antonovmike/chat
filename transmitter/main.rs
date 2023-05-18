use std::net::TcpStream;
use sqlite::Error;

mod transmitter_lib;
mod transmitter;

const LOCAL: &str = "127.0.0.1:6000";
// const STRUCT_SIZE: usize = 96;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = TcpStream::connect(LOCAL)
        .expect("Stream failed to connect");
    client.set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
        transmitter::transmitter(client)?;
        
        Ok(())
}
