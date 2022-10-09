use std::net::TcpListener;

mod receiver_lib;
mod receiver;

const LOCAL: &str = "127.0.0.1:6000";

fn main() {
    let server = TcpListener::bind(LOCAL)
        .expect("Listener failed to bind");
    server.set_nonblocking(true)
        .expect("Failed to initialize non-blocking");
    receiver::receiver(server);
}
