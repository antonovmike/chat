use sqlite::Error;
use std::net::TcpStream;

mod transmitter;
mod transmitter_lib;

const LOCAL: &str = "127.0.0.1:8080";
// const STRUCT_SIZE: usize = 96;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    transmitter::transmitter(client).await?;

    Ok(())
}
