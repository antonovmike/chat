use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::lib::{UserData, UserID};
use colored::Colorize;
use serde_json;

mod lib;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
const USER_NAME_SIZE: usize = 16;
const STRUCT_SIZE: usize = 96;

// #[tokio::main]
/*async*/ fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            let tx1 = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buff_serde = vec![0; STRUCT_SIZE];
                match socket.read_exact(&mut buff_serde) {
                    Ok(_) => {
                        let serde_content = buff_serde
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        let serde_message = String::from_utf8(serde_content).expect("Invalid utf8 message");

                        let deserialized: UserData = serde_json::from_str(&serde_message).expect("Could not read");

                        let user_id: UserID = lib::UserID {
                            id: addr.to_string(),
                            data: deserialized,
                        };

                        println!("{} {} {} \n{}", 
                            format!("{}", user_id.data.name).bold().yellow(),
                            format!("(ID: {})", user_id.id).bold().purple(),
                            format!("said:").bold().yellow(),
                            format!("\"{}\"", user_id.data.message).italic().on_green()
                        );

                        tx1.send(serde_message).expect("Failed to send message to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("{}", format!("Closing connection with {}", addr).on_purple());
                        break;
                    }
                }
                sleep();
            });
        }

        if let Ok(user_message) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff_message = user_message.clone().into_bytes();
                buff_message.resize(MSG_SIZE, 0);
                client.write_all(&buff_message).map(|_| client).ok()
            })
                .collect::<Vec<_>>();
        }
        
        if let Ok(user_name) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff_name = user_name.clone().into_bytes();
                buff_name.resize(USER_NAME_SIZE, 0);
                client.write_all(&buff_name).map(|_| client).ok()
            })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}
