extern crate chat_group;
extern crate client;

use std::net::TcpListener;
use std::thread;
use client::*;
use chat_group::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    
    println!("Start to listen, ready to accept");

    let sample_chat_group = ChatGroup::new();
    let _tx = sample_chat_group.get_transmitter().unwrap();

    thread::spawn(move|| {
        handle_chat_group(sample_chat_group);
    });

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(new_stream) => {
                println!("[DEBUG][STREAM] new stream");

                let new_client = Client::new(new_stream, Some(_tx.clone()));
                _tx.send(Signal::NewClient(new_client));
            }
            Err(_) => { println!("connection failed"); }
        }
    }
    
    // close the socket server
    drop(listener);
}