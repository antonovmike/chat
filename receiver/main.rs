use std::net::TcpListener;
use sqlite::Error;

mod receiver;
mod receiver_lib;

const LOCAL: &str = "127.0.0.1:6000";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let server = TcpListener::bind(LOCAL)
        .expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");
    receiver::receiver(server)?;
    Ok(())
}
