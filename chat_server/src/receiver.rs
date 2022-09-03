use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use colored::Colorize;
use serde_json;
use std::collections::HashMap;

const DATA_SIZE: usize = 96;

fn sleep() { thread::sleep(Duration::from_millis(100)); }

pub fn receiver(server: TcpListener) {
    let mut clients = vec![];
    let (tx, _rx) = mpsc::channel::<String>();
    
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            let tx1 = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buff_serde = vec![0; DATA_SIZE];
                match socket.read_exact(&mut buff_serde) {
                    Ok(_) => {
                        let serde_content = buff_serde
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        let serde_message = String::from_utf8(serde_content).expect("Invalid utf8 message");

                        let deserialized: HashMap<String, String> = serde_json::from_str(&serde_message).expect("Could not read");

                        for (key, value) in &deserialized {
                            println!("{} (ID: {}) {} \n{}", 
                                format!("{}", key).bold().yellow(), 
                                format!("(ID: {})", addr.to_string()).bold().purple(), 
                                format!("said:").bold().yellow(),
                                format!("\"{}\"", value).italic().on_green());
                        }

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

        sleep();
    }
}