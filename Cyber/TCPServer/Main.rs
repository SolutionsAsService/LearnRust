use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Loop to keep connection alive and echo back messages
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                // Simulate processing delay
                thread::sleep(Duration::from_secs(1));
                
                // Echo back the received message
                stream.write(&buffer[..size]).unwrap();
                println!("Received and echoed back: {}", String::from_utf8_lossy(&buffer[..size]));
            },
            Err(e) => {
                println!("An error occurred while reading from connection: {}", e);
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            },
        }
    }
}
