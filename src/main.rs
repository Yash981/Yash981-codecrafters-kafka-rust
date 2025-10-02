#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(strm) => {
                let mut buf = [0u8; 1024];
                let mut streamm = strm;
                let _ = streamm.read(&mut buf).unwrap();
                let response: [u8; 8]   = [
                    0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x07,
                ];
                streamm.write_all(&response).unwrap();
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
