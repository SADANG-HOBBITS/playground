use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::thread::sleep_ms;
use std::sync::{Arc, Mutex};

extern crate bincode;
extern crate rustc_serialize;

pub const message_tag_health_check: u64 = 0x0;
pub const message_tag_issue_id: u64 = 0x1;
pub const message_tag_chat: u64 = 0x2;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Header {
    pub length: usize,
    pub message_tag: u64,
    pub group_id: u64,
    pub sender_id: u64,
}

struct Client {
    pub id: u64,
    pub stream: TcpStream,
    pub recv_buf : [u8; 2048],
    pub read_buf : [u8; 128],
    pub end_idx: usize,
}

impl Client {
    pub fn read_message(&mut self) -> Option<String> {
        println!("start to read");
        
        let recv_size = self.stream.read(&mut self.read_buf).unwrap();
        
        println!("finished");
        
        if recv_size > 0 {
            for idx in 0..recv_size {
                self.recv_buf[self.end_idx] = self.read_buf[idx];
                self.end_idx += 1;
            }
            
            let (body_end, message) = self.handle_server_message();
            
            // shift
            if body_end > 0 {
                for idx in 0..(self.end_idx-body_end) {
                    self.recv_buf[idx] = self.recv_buf[body_end+idx];
                }
                
                self.end_idx -= body_end;
            
                return message;
            }
        }

        None
    }

    fn handle_server_message(&mut self) -> (usize, Option<String>) {
        let header: Header = bincode::decode(&self.recv_buf[0..32]).unwrap();
        let body_end = (header.length + 32) as usize;
        
        if self.end_idx < body_end { 
            return (0, None); 
        }
        
        match header.message_tag {
            message_tag_chat => {
                let message = String::from_utf8_lossy(&self.recv_buf[32..body_end]);
                // println!("{}", message);
                
                println!("[{}]>>> {}", header.sender_id, message);
                
                (body_end, Some(message.to_string()))
            },
            message_tag_issue_id => {
                self.id = header.sender_id;
                
                (body_end, None)
            },
            _ => { (body_end, None) },
        }
    }

    pub fn send_message(&mut self, header: &mut Header, message: &[u8]) {
        header.sender_id = self.id;
        
        let header_bytes = bincode::encode(header, bincode::SizeLimit::Infinite).unwrap();
        
        let mut sent = self.stream.write(&header_bytes[..]).unwrap();
        sent += self.stream.write(message).unwrap();
        
        match self.stream.flush() {
            Ok(_) => println!("[send] {:?} bytes", sent),
            _ => {},
        }
    }
}

fn fetch_from_server(handle: Arc<Mutex<Client>>) {
    loop {
        let mut client_reader = handle.lock().unwrap();   
        client_reader.read_message();
    }
}

fn main() {
    let mut new_stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    
    println!("connected");
    
    let mut client = Arc::new(Mutex::new(Client{ id:0, stream: new_stream, recv_buf: [0;2048], read_buf: [0;128], end_idx: 0 }));
    let clone = client.clone();
    
    let read_thread = thread::spawn(move|| { 
        fetch_from_server(clone);
    });
    
    let clone = client.clone();
    loop {
        let mut user_input: String = String::new();
	    io::stdin().read_line(&mut user_input);
        
        println!("before sending message");
        
        // make header
        let mut header = Header{ length:0, message_tag:message_tag_chat, group_id:0, sender_id: 0 };
        header.length = user_input.len();
        
        println!("before lock");
        
        let mut client_writer = clone.lock().unwrap(); 
        
        println!("after lock");        
        
        client_writer.send_message(&mut header, &user_input.into_bytes()[..]);
        
        println!("after sending message");
    }
    
    let result = read_thread.join();
   
    println!("Close the connection");
}