use std::io::Write;
use std::net::TcpStream;
use crate::mqtt::message_type::MessageType;

// Returns an answer to the client
pub fn send_response(stream: &mut TcpStream, buffer: &[u8], ) {
    println!("Sending message: {:?}", buffer);
    stream.write(&buffer);
}
