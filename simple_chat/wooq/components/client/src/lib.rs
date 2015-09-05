extern crate header;
extern crate bincode;
extern crate rustc_serialize;

use std::io::prelude::*;
use bincode::rustc_serialize::{encode, decode};

use std::net::TcpStream;
use header::*;

pub fn send_message(mut stream: &TcpStream, header: Header, message: Option<String>) {
    if header.length > CLIENT_BUFFER_SIZE-HEADER_SIZE { return; }

    let header_bytes = encode_header(&header);
    stream.write(&header_bytes[..]).unwrap();
    
    match message {
        Some(content) => {
            let message_bytes = content.into_bytes();
            stream.write(&message_bytes[..]).unwrap();
        },
        _ => {},
    }

    stream.flush();
}

pub const CLIENT_BUFFER_SIZE: usize = 512;
const CLOSE_CODE: usize = !0;

pub struct Client {
    pub id: u64,
    pub stream: TcpStream,
    pub recv_buf : [u8; CLIENT_BUFFER_SIZE],
    pub read_buf : [u8; CLIENT_BUFFER_SIZE],
    pub end_idx: usize,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client{ id:0, stream: stream, recv_buf: [0;CLIENT_BUFFER_SIZE], read_buf: [0;CLIENT_BUFFER_SIZE], end_idx: 0 }
    }

    /// 챗 그룹에서 메시지 전송할 때 사용할 스트림 반환 
    pub fn get_write_stream(&self) -> Option<TcpStream> {
        match self.stream.try_clone() {
            Ok(stream) => { Some(stream) },
            _ => { None }
        }
    }

    pub fn read_message(&mut self) -> bool {
        match self.stream.read(&mut self.read_buf) {
            Ok(recv_size) => {
                if recv_size == 0 { return false; }

                for idx in 0..recv_size {
                    self.recv_buf[self.end_idx] = self.read_buf[idx];
                    self.end_idx += 1;
                }
                
                let (body_end, sender_id, message) = self.handle_received_message();
                
                // shift
                match body_end {
                    CLOSE_CODE => { return false; },
                    0 => {},
                    _ => {
                        for idx in 0..(self.end_idx-body_end) {
                            self.recv_buf[idx] = self.recv_buf[body_end+idx];
                        }
                        
                        self.end_idx -= body_end;
                    
                        match message {
                            Some(chat_message) => {
                                println!("{} >>> {}", sender_id, chat_message);
                            },
                            None => {},
                        }
                    }
                }

                return true;
            },
            Err(_) => { 
                println!("[DEBUG] read stream error");
                return false; 
            }
        }
    }

    fn handle_received_message(&mut self) -> (usize, u64, Option<String>) {
        let header: Header = decode_header(&self.recv_buf[0..HEADER_SIZE]);
        let body_end = (header.length + HEADER_SIZE) as usize;
        
        if self.end_idx < body_end { return (0, 0, None); }
        
        match header.message_tag {
            MESSAGE_TAG_ISUUE_ID => {
                self.id = header.sender_id;
                (body_end, 0, None)
            },
            MESSAGE_TAG_CHAT => {
                let message = String::from_utf8_lossy(&self.recv_buf[HEADER_SIZE..body_end]);
                (body_end, header.sender_id, Some(message.to_string()))
            },
            MESSAGE_TAG_CLOSE => {
                println!("close the client please...");
                (CLOSE_CODE, 0, None)
            },
            _ => { (body_end, 0, None) },
        }
    }
}

pub fn fetch_from_server(mut client: Client) {
    loop {
        if (client.read_message()) { continue; }
        else { break; }
    }
}

