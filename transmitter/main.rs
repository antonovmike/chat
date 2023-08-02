// use std::net::TcpStream;

use sqlite::Error;

extern crate gtk;
use gtk::{prelude::*, Application};

use client::*;

mod client;
mod db;
mod gui;
mod transmitter;
mod transmitter_lib;

// const LOCAL: &str = "127.0.0.1:8080";
// const STRUCT_SIZE: usize = 96;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let application = Application::builder()
        .application_id("com.example.ChatApp")
        .build();

    application.connect_activate(gui::build_ui);

    application.run();

    start_client().await?;

    Ok(())
}
