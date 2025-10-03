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
            Ok(mut strm) => {
                let mut header_buf = [0; 512];
                let _ = strm.read(&mut header_buf);
                let correlation_id: i32 = i32::from_be_bytes(header_buf[8..12].try_into().unwrap());
                println!("correlation_id: {}",correlation_id);
                let _ = strm.write_all(&0_i32.to_be_bytes());
                let _ = strm.write_all(&correlation_id.to_be_bytes());    
                }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
