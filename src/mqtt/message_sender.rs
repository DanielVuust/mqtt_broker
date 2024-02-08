use std::io::Write;
use std::net::TcpStream;

use super::message_type::MessageType;

// Returns an answer to the client
pub fn send_response(stream: &mut TcpStream, buffer: &[u8], ) {
    println!("SENT MESSAGE: {:?}", buffer);

    // Sends message and checks errors
    match stream.write(buffer) {
        Ok(_) => {
            //println!("Message sent");
        }
        Err(e) => {
            println!("Failed to send message: {}", e);
        }
    }

    // Flushes the stream
    match stream.flush() {
        Ok(_) => {
            //println!("Stream flushed");
        }
        Err(e) => {
            println!("Failed to flush stream: {}", e);
        }
    }
}

// Respond to the client with a response packet (puback, pubrec, pubrel, pubcomp, suback, unsuback)
pub fn send_response_packet(stream: &mut TcpStream, message_type: MessageType, packet_identifier: u16) {
    let mut response: [u8; 4] = [0; 4];
    response[0] = message_type.to_u8();
    response[1] = 0x02;


    // Transforms the packet identifier to u8
    let (msb, lsb) = transform_package_identifier_to_u8(packet_identifier);

    response[2] = msb; // MSB of packet identifier
    response[3] = lsb; // LSB of packet identifier

    send_response(stream, &response);
}

// Function to extract packet identifier based on QoS
pub fn get_packet_identifier_to_u16(buffer: &[u8], index: usize) -> u16 {

    // TODO: Remove this print
    //println!("PACKET IDENTIFIER: MSB: {}, LSB: {}", buffer[index], buffer[index + 1]);

    // Adds the MSB and LSB to get the packet identifier in u16
    let packet_identifier = ((buffer[index] as u16) << 8) | (buffer[index + 1] as u16);

    // TODO: Remove this print
    //println!("CONVERTED TO: {}", packet_identifier);

    packet_identifier
}

// Function to transform packet identifier to u8 (MSB and LSB)
pub fn transform_package_identifier_to_u8(packet_identifier: u16) -> (u8, u8) {
    let mut response: [u8; 2] = [0; 2];

    // TODO: Remove this print
    //println!("PACKET IDENTIFIER: {}", packet_identifier);

    // Transforms the packet identifier to u8
    response[0] = (packet_identifier >> 8) as u8; // MSB of packet identifier
    response[1] = (packet_identifier & 0xFF) as u8; // LSB of packet identifier

    // TODO: Remove this print
    //println!("CONVERTED TO: MSB: {}, LSB: {}", response[0], response[1]);

    (response[0], response[1])
}