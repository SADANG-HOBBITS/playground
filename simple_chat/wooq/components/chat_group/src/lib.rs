extern crate bincode;
extern crate rustc_serialize;
extern crate client;
extern crate header;

use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::collections::HashMap;
use client::*;
use header::*;

pub struct ChatGroup {
    pub client_stream_map: HashMap<u64, TcpStream>,
    pub last_issued_client_id: u64,
    pub tx: Sender<Signal>,
    pub rx: Receiver<Signal>
}

impl ChatGroup {
    pub fn new() -> ChatGroup {
        let (_tx, _rx): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        ChatGroup{ client_stream_map: HashMap::new(), last_issued_client_id: 0, tx: _tx, rx: _rx }
    }

    pub fn get_transmitter(&self) -> Option<Sender<Signal>> {
        Some(self.tx.clone())
    }

    pub fn cycle(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(signal) => {
                    match signal {
                        Signal::NewClient(mut new_client) => {
                            self.last_issued_client_id += 1;

                            new_client.id = self.last_issued_client_id;
                            let client_stream = new_client.get_write_stream().unwrap();
                            send_message(&client_stream, &Header::new(0, MESSAGE_TAG_ISUUE_ID, self.last_issued_client_id), None);

                            self.client_stream_map.insert(self.last_issued_client_id, client_stream);
                            thread::spawn(move|| {
                                read_loop(new_client);
                            });
                        },
                        Signal::NewMessage(new_header, new_message) => {
                            for (id, each_client_stream) in &mut self.client_stream_map {
                                send_message(&each_client_stream, &new_header, Some(new_message.clone()));
                            }
                        },
                        Signal::Close(id) => {
                            send_message(&self.client_stream_map.get(&id).unwrap(), &Header::new(0, MESSAGE_TAG_CLOSE, id), None);
                            self.client_stream_map.remove(&id);
                        }
                    }
                },
                _ => { break; }
            }
        }
    }
}

pub fn handle_chat_group(mut chat_group: ChatGroup) {
    chat_group.cycle();
}