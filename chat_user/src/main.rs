use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use colored::Colorize;

mod lib;

const LOCAL: &str = "127.0.0.1:6000";
const STRUCT_SIZE: usize = 96;

fn main() {
    println!("{}", "Enter your name".bold().on_yellow());
	let mut user_name = String::new();
	io::stdin().read_line(&mut user_name).expect("Failed to read line");
    user_name.pop();

    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    // SENDS DATA TO SERVER
    thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(user_message) => {
                let user_data = lib::UserData {
                    name: user_name.clone(),
                    message: user_message.clone(),
                };

                let serialized = serde_json::to_string(&user_data)
                    .unwrap()
                    .clone()
                    .into_bytes();

                let mut buff_serde = serialized.clone();
                buff_serde.resize(STRUCT_SIZE, 0);

                client.write_all(&buff_serde).expect("Writing to socket failed");
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("{}", "Write a message:".bold().on_green());
    loop{
        let mut buff_message = String::new();
        io::stdin().read_line(&mut buff_message).expect("Reading from stdin failed");
        let user_message = buff_message.trim().to_string();
        if user_message == ":quit" || tx.send(user_message).is_err() { break }
    }
    println!("{}", "Bye".bold().on_blue());

    loop{
        let mut buff_name = String::new();
        io::stdin().read_line(&mut buff_name).expect("Reading from stdin failed");
        let user_name = buff_name.trim().to_string();
        if user_name == ":quit" || tx.send(user_name).is_err() { break }
    }
}