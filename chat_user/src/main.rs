#![allow(unused)]
use std::io::{self, /*ErrorKind, Read,*/ Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
// use futures::prelude::*;
// use tokio::prelude::*;
// use tokio::io::AsyncWriteExt;
// use tokio::net::TspStream;
use colored::Colorize;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
const USER_NAME_SIZE: usize = 16;

fn parse(a: String) -> Vec<u8> {
    a.chars().filter(|b| *b != '\n').map(|c| c as u8).collect()
}
fn main() {
    println!("{}", "Enter your name".bold().on_green());
	let mut user_name = String::new();
	io::stdin().read_line(&mut user_name).expect("Failed to read line");
    user_name.pop();
	
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();
    let new_tx = tx.clone();

    thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(user_message) => {
                let mut buff_message = user_message.clone().into_bytes();
                buff_message.resize(MSG_SIZE, 0);
                client.write_all(&buff_message).expect("Writing to socket failed");
                println!("{}", format!("{}: {:?}", user_name, user_message).bold().on_blue());
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
        let mut user_message = buff_message.trim().to_string();
        if user_message == ":quit" || tx.send(user_message).is_err() { break }
    }
    println!("{}", "Bye".bold().on_blue());

    loop{
        let mut buff_name = String::new();
        io::stdin().read_line(&mut buff_name).expect("Reading from stdin failed");
        let mut user_name = buff_name.trim().to_string();
        if user_name == ":quit" || tx.send(user_name).is_err() { break }
    }
}
