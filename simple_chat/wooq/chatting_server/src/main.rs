use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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
    pub fn get_write_stream(&self) -> Option<TcpStream> {
        self.stream.try_clone();
    }

    pub fn read_message(&mut self) -> Option<String> {
        println!("client added");

        let recv_size = self.stream.read(&mut self.read_buf).unwrap();
        
        println!("after read");

        if recv_size > 0 {
            for idx in 0..recv_size {
                self.recv_buf[self.end_idx] = self.read_buf[idx];
                self.end_idx += 1;
            }
            
            let (body_end, message) = self.handle_client_message();
            
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

    fn handle_client_message(&mut self) -> (usize, Option<String>) {
        let header: Header = bincode::decode(&self.recv_buf[0..32]).unwrap();
        let body_end = (header.length + 32) as usize;
        
        if self.end_idx < body_end { 
            return (0, None); 
        }
        
        match header.message_tag {
            message_tag_chat => {
                let message = String::from_utf8_lossy(&self.recv_buf[32..body_end]);
                // println!("{}", message);
                
                (body_end, Some(message.to_string()))
            },
            _ => { (body_end, None) },
        }
    }

    pub fn send_message(&mut self, header: &Header, message: &[u8]) {
        if header.sender_id == self.id { return; }

        let header_bytes = bincode::encode(header, bincode::SizeLimit::Infinite).unwrap();
        
        let mut sent = self.stream.write(&header_bytes[..]).unwrap();
        sent += self.stream.write(message).unwrap();
        
        match self.stream.flush() {
            Ok(_) => println!("[send] {:?} bytes", sent),
            _ => {},
        }
    }
}

struct ChatGroup {
    pub id: u64,
    pub clients: Vec<Client>,
    pub wait_queue: Vec<Client>,
    pub message_queue: Vec<(Header, String)>,
    pub last_issued_client_id: u64,
}

impl ChatGroup {
    pub fn add_client(&mut self, mut new_client: Client) {
        self.last_issued_client_id += 1;

        new_client.id = self.last_issued_client_id;
        self.wait_queue.push(new_client);

        println!("client added");
    }

    pub fn cycle(&mut self) {
        println!("cycle started");

        // check new clients
        while self.wait_queue.len() > 0 {
            match self.wait_queue.pop() {
                Some(new_client) => {
                    self.clients.push(new_client);
                },
                _ => {},
            }
        }

        // update new messgaes
        for each_client in &mut self.clients {
            match each_client.read_message() {
                Some(message) => {
                    // build header
                    let mut header = Header{ length:0, message_tag:message_tag_chat, group_id:self.id, sender_id: each_client.id };
                    header.length = message.len();

                    self.message_queue.push((header, message));
                },
                _ => {}
            }
        }

        // broadcast messages
        while self.message_queue.len() > 0 {
            match self.message_queue.pop() {
                Some(message_set) => {
                    let (new_header, new_message) = message_set;

                    for each_client in &mut self.clients {
                        // TODO: avoid to clone the new message
                        each_client.send_message(&new_header, &new_message.clone().into_bytes()[..]);
                    }
                },
                _ => {},
            }
        }

        println!("cycle ended");
    }
}

/*
fn client_health_check(mut stream: TcpStream) {
    println!("Start health check for the new connection");

    let mut count = 0;

    loop {
        let message = format!("Health check {}", count);
        let mut header = Header{ length:0, message_id:2, group_id:0 };
        header.length = message.len();
        let header_bytes = bincode::encode(&header, bincode::SizeLimit::Infinite).unwrap();

        stream.write(&header_bytes[..]);
        stream.write(&message.into_bytes()[..]);
        stream.flush();

        sleep_ms(5000);
        count += 1;
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("Start to handle the new connection");

    let mut recv_buf : [u8; 2048] = [0;2048];
    let mut end_idx = 0;

    loop {
        let mut read_buf : [u8; 128] = [0;128];
        let recv_size = stream.read(&mut read_buf).unwrap();

        if recv_size > 0 {
            // println!("[DEBUG][recv={}]", recv_size);
            // println!("[DEBUG] {}", String::from_utf8_lossy(&read_buf[..]));
            
            for idx in 0..recv_size {
                recv_buf[end_idx] = read_buf[idx];
                end_idx += 1;
            }
            



            // shift
            if body_end > 0 {
                for idx in 0..(end_idx-body_end) {
                    recv_buf[idx] = recv_buf[body_end+idx];
                }
                
                end_idx -= body_end;
            }
        }

        let mut header = Header{ length:0, message_id:1, group_id:0 };
        header.length = recv_size;
        let header_bytes = bincode::encode(&header, bincode::SizeLimit::Infinite).unwrap();

        stream.write(&header_bytes[..]);
        stream.write(&read_buf[0..recv_size]);
        stream.flush();
    }
}
*/

fn handle_chat_group(handle: Arc<Mutex<ChatGroup>>) {
    loop {
        let mut chat_group = handle.lock().unwrap();
        chat_group.cycle();
    }
}

#[allow(unstable)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    
    println!("Start to listen, ready to accept");

    let mut sample_chat_group = Arc::new(Mutex::new(ChatGroup{ id: 0, clients: Vec::new(), wait_queue: Vec::new(), message_queue: Vec::new(), last_issued_client_id: 0 }));
    let clone = sample_chat_group.clone();
    
    thread::spawn(move|| {
        handle_chat_group(clone);
    });


    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(new_stream) => {
                let mut new_client = Client{ id:0, stream: new_stream, recv_buf: [0;2048], read_buf: [0;128], end_idx: 0 };

                {
                    let clone = sample_chat_group.clone();
                    let mut chat_group = clone.lock().unwrap();

                    chat_group.add_client(new_client);
                }                
            }
            Err(e) => { /* connection failed */ }
        }
    }
    
    // close the socket server
    drop(listener);
}