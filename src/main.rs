#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

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
                let mut streamm = strm;
                let mut header_buf = [0u8; 512];
                streamm.read_exact(&mut header_buf).unwrap();
                let correlation_id: i32 = i32::from_be_bytes(header_buf[8..12].try_into().unwrap());

                streamm.write_all(&0i32.to_be_bytes()).unwrap();
                streamm.write_all(&correlation_id.to_be_bytes()).unwrap(); // correlation_id    
                }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
