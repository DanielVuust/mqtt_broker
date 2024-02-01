use std::io::Write;
use std::net::TcpStream;
use crate::mqtt::message_type::MessageType;

// Returns an answer to the client
pub fn send_response(stream: &mut TcpStream, buffer: &[u8], ) {
    stream.write(&buffer).unwrap();
}

pub fn generate_package_type_byte(message_type: MessageType) -> u8 {
    message_type.to_u8() << 4
}