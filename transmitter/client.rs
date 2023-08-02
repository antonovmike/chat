use std::net::TcpStream;
use sqlite::Error;
use crate::transmitter;

const LOCAL: &str = "127.0.0.1:8080";

pub async fn start_client() -> Result<(), Error> {
    let client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    transmitter::transmitter(client).await?;

    Ok(())
}