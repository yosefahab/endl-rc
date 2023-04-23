#![allow(unused)]
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn get_invite_link() -> String {
    String::from("")
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream.try_clone().unwrap();

    // Send welcome message
    writer.write_all(b":localhost 001 welcome\r\n").unwrap();

    loop {
        let mut message = String::new();
        reader.read_line(&mut message).unwrap();

        // Parse message
        let parts: Vec<&str> = message.trim().split(' ').collect();
        let command = parts[0];

        match command {
            "JOIN" => {
                // Handle join command
            }
            "PRIVMSG" => {
                // Handle private message
            }
            _ => {
                // Handle unknown command
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6667").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting client: {}", e);
            }
        }
    }
}
