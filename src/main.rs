#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                loop {
                    let mut buf = [0; 512];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            stream.write_all(b"+PONG\r\n").unwrap();
                        }
                        Err(e) => {
                            eprintln!("Failed to read from client: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
