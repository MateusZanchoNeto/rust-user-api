use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::{constants::HOST, controllers::Controllers, database::Database};

#[macro_use]
extern crate serde_derive;

//mods
mod constants;
mod controllers;
mod database;
mod user;

fn main() {
    // Setting database
    match Database::new() {
        Ok(mut database) => {
            database.set_database().unwrap();
        }
        Err(e) => {
            println!("Error setting database: {}", e);
            return;
        }
    }

    // Starting the API server
    let listener = TcpListener::bind(HOST).unwrap();
    println!("Server listening on {}", HOST);

    // Handling the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    let mut controllers = Controllers::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            // Handling the request and sending the response
            controllers.handle(stream, request);
        }
        Err(e) => println!("Error: {}", e),
    }
}
