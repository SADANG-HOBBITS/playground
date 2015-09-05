extern crate bincode;
extern crate rustc_serialize;
extern crate client;
extern crate header;

use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use bincode::rustc_serialize::{encode, decode};
use client::*;
use header::*;

const COMMAND_TERMINATION: &'static str = "/exit\n";

fn main() {
    let mut new_stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    
    println!("connected");
    
    let mut client = Client::new(new_stream);
    let mut write_stream = client.get_write_stream().unwrap();
    
    let read_thread = thread::spawn(move|| { 
        fetch_from_server(client);
    });
    
    loop {
        let mut user_input: String = String::new();
	    io::stdin().read_line(&mut user_input);
        
        if user_input == COMMAND_TERMINATION {
            println!("exit....");
            send_message(&write_stream, Header::new(0, MESSAGE_TAG_CLOSE, 0), None);
            break;
        }

        send_message(&write_stream, Header::new(user_input.len(), MESSAGE_TAG_CHAT, 0), Some(user_input));
    }
    
    let result = read_thread.join();
   
    println!("Close the connection");
}