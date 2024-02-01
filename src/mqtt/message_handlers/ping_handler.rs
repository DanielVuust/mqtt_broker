use std::{io::Write, net::TcpStream};

use crate::mqtt::message_type::MessageType;

pub fn ping_resp(stream: &mut TcpStream, msq_type: MessageType) {
    println!("Trying to send ping responds");
    let packet: &[u8] = &[0xd0,0];
    stream.write(packet);
    println!("Ping responds has been successfully send to client");
}