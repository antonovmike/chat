use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::receiver_lib::{UserData, UserID};
use colored::Colorize;
use sqlite::State;

const DATA_SIZE: usize = 96;

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}

pub fn receiver(server: TcpListener) {
    let connection = sqlite::open(":memory").unwrap();
    let drop = "DROP TABLE users";
    connection.execute(drop).unwrap();

    
    let mut clients = vec![];
    let (tx, _rx) = mpsc::channel::<String>();
    
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            let tx1 = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let connection = sqlite::open(":memory").unwrap();
                let query = "CREATE TABLE if NOT EXISTS users (name CHARFIELD, message TEXT)";
                connection.execute(query).unwrap();                let mut buff_serde = vec![0; DATA_SIZE];
                match socket.read_exact(&mut buff_serde) {
                    Ok(_) => {
                        let serde_content = buff_serde
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        let serde_message = String::from_utf8(serde_content).expect("Invalid utf8 message");

                        let deserialized: UserData = serde_json::from_str(&serde_message).expect("Could not read");

                        let user_id: UserID = UserID {
                            id: addr.to_string(),
                            data: deserialized,
                        };
                        
                        let query = format!("INSERT INTO users VALUES ('{}', '{}')", user_id.data.name, user_id.data.message);
                        connection.execute(query).unwrap();
                        
                        let query = "SELECT * FROM users";
                        let mut statement = connection.prepare(query).unwrap();
                        while let Ok(State::Row) = statement.next() {
                            println!("{} said: {}", 
                                statement.read::<String, _>("name").unwrap(),
                                statement.read::<String, _>("message").unwrap()
                            )
                        }

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

        sleep();
    }
}
