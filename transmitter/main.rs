use sqlite::Error;
use std::net::TcpStream;
// use std::thread;

mod transmitter;
mod transmitter_lib;
mod tui;
mod db;

const LOCAL: &str = "127.0.0.1:8080";
// const STRUCT_SIZE: usize = 96;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // thread::spawn(|| {
    //     tui::run_ui().expect("Failed to run UI");
    // });

    let client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    transmitter::transmitter(client).await?;

    Ok(())
}
