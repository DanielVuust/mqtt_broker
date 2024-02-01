use std::net::TcpStream;

use crate::mqtt::{message_sender::{generate_package_type_byte, send_response}, message_type::MessageType};

//This function has the responsibility for sending a PINGRESP to the client
pub fn ping_resp(stream: &mut TcpStream, message_type: MessageType) {
    println!("Trying to send PINGRESP");

    //Create array of two bites.
    //PingResp only uses two bytes
    let mut response: [u8; 2] = [0; 2];
    response[0] = generate_package_type_byte(message_type);
    response[1] = 0;
    
    send_response(stream, &response);

    println!("PINGRESP has been successfully send to client");
}