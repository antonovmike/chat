use crate::transmitter_lib::UserData;
use colored::Colorize;
use sqlite::State;
use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const DATA_SIZE: usize = 96;

pub fn transmitter(mut client: TcpStream) {
    let connection = sqlite::open("chat.db").unwrap();
    let query = "CREATE TABLE if NOT EXISTS users (name CHARFIELD, message TEXT)";
    connection.execute(query).unwrap();

    println!("Previous messages:");
    let query = "SELECT * FROM users ORDER BY ROWID DESC LIMIT 5";
    let mut statement = connection.prepare(query).unwrap();
    let mut prev_mess: Vec<String> = vec![];
    while let Ok(State::Row) = statement.next() {
        // println!(
        //     "{} {}",
        //     format!("{} said:", statement.read::<String, _>("name").unwrap())
        //         .bold()
        //         .yellow(),
        //     format!("{}", statement.read::<String, _>("message").unwrap())
        //         .bold()
        //         .green()
        // )
        prev_mess.push(format!(
            "{} said: {}",
            statement.read::<String, _>("name").unwrap(),
            statement.read::<String, _>("message").unwrap()
        ))
    }
    // println!("{:?}", prev_mess);
    prev_mess.reverse();
    for m in prev_mess {
        println!("{}", m.bold().green())
    }

    let query = "SELECT * FROM users";
    let mut statement = connection.prepare(query).unwrap();
    let mut index = 0;
    while let Ok(State::Row) = statement.next() {
        index += 1
    }

    println!("{}", "Enter your name".bold().on_yellow());
    let mut user_name = String::new();
    io::stdin()
        .read_line(&mut user_name)
        .expect("Failed to read line");
    user_name.pop();

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let connection = sqlite::open("chat.db").unwrap();
        let query = "SELECT * FROM users";
        let mut statement = connection.prepare(query).unwrap();
        let mut index_inner = 0;
        while let Ok(State::Row) = statement.next() {
            index_inner += 1
        }
        if index_inner > index {
            let query = "SELECT * FROM users ORDER BY ROWID DESC LIMIT 1";
            let mut statement = connection.prepare(query).unwrap();
            while let Ok(State::Row) = statement.next() {
                println!(
                    "{} {}",
                    format!("{} said:", statement.read::<String, _>("name").unwrap())
                        .bold()
                        .yellow(),
                    format!("{}", statement.read::<String, _>("message").unwrap())
                        .bold()
                        .green()
                )
            }
            index += 1
        }

        match rx.try_recv() {
            Ok(user_message) => {
                let user_data = UserData {
                    name: user_name.clone(),
                    message: user_message.clone(),
                };

                let serialized = serde_json::to_string(&user_data)
                    .unwrap()
                    .clone()
                    .into_bytes();

                let mut buff_serde = serialized.clone();
                buff_serde.resize(DATA_SIZE, 0);

                client
                    .write_all(&buff_serde)
                    .expect("Writing to socket failed");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("{}", "Write a message:".bold().on_green());
    loop {
        let mut buff_message = String::new();
        io::stdin()
            .read_line(&mut buff_message)
            .expect("Reading from stdin failed");
        let user_message = buff_message.trim().to_string();
        if user_message == ":quit" || tx.send(user_message).is_err() {
            break;
        }
    }
    println!("{}", "Bye".bold().on_blue());
}
