extern crate bincode;
extern crate rustc_serialize;

use bincode::rustc_serialize::{encode, decode};

pub const HEADER_SIZE: usize = 24;

pub const MESSAGE_TAG_ISUUE_ID: u64 = 0x0;
pub const MESSAGE_TAG_CHAT: u64 = 0x1;
pub const MESSAGE_TAG_CLOSE: u64 = 0x2;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Header {
    pub length: usize,
    pub message_tag: u64,
    pub sender_id: u64,
}

impl Header {
	pub fn new(length: usize, message_tag: u64, sender_id: u64) -> Header {
		Header{ length: length, message_tag: message_tag, sender_id: sender_id }
	}
}

pub fn encode_header(header: &Header) -> Vec<u8> {
	encode(header, bincode::SizeLimit::Infinite).unwrap()
}

pub fn decode_header(bytes: &[u8]) -> Header {
	decode(bytes).unwrap()
}