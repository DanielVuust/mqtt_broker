use std::io::Write;
use std::net::TcpStream;
use crate::mqtt::message_type::MessageType;

// Returns an answer to the client
pub fn send_answer(stream: &mut TcpStream, msg_type: MessageType) {
    let mut connack_buf = [32, 2, 0, 0];
    connack_buf[0] = msg_type.to_u8() << 4;
    stream.write(&connack_buf).unwrap();
}