mod message_type;
use message_type::MessageType;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// Handles client connection
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Reads data from stream until connection is closed
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // No data received, closing connection
                false
            } else {
                // Uses bit-shift operation to move first 4 bits to the right of the byte and converts it to u8 
                // Example: '0001 1010' >> 4 = '0000 0001' = 1u8
                let msg_type = buffer[0] >> 4;

                // Handles client message types
                match MessageType::from_u8(msg_type) {
                    // msg_type = Connect
                    Some(MessageType::Connect) => {
                        match parse_connect_message(&buffer[..size]) {
                            Ok(_) => println!("CONNECT message parsed successfully"),
                            Err(e) => println!("Error parsing CONNECT message: {}", e),
                        }
                    }
                    // msg_type = Publish
                    Some(MessageType::Publish) =>{
                        println!("PUBLISH message received");
                    }
                    // msg_type = Puback
                    Some(MessageType::Puback) =>{
                        println!("PUBACK message received");
                    }
                    // msg_type = Pubrec
                    Some(MessageType::Pubrec) =>{
                        println!("PUBREC message received");
                    }
                    // msg_type = Pubrel
                    Some(MessageType::Pubrel) =>{
                        println!("PUBREL message received");
                    }
                    // msg_type = Pubcomp
                    Some(MessageType::Pubcomp) =>{
                        println!("PUBCOMP message received");
                    }
                    // msg_type = Subscribe
                    Some(MessageType::Subscribe) =>{
                        println!("SUBSCRIBE message received");
                    }
                    // msg_type = Unsubscribe
                    Some(MessageType::Unsubscribe) =>{
                        println!("Unsubscribe message received");
                    }
                    // msg_type = Pingreq
                    Some(MessageType::Pingreq) =>{
                        println!("PINGREQ message received");
                    }
                    // msg_type = Disconnect
                    Some(MessageType::Disconnect) => {
                        println!("DISCONNECT message received");
                        return;
                    }
                    // msg_type = Invalid or unsupported
                    _ => {
                        println!("Invalid or unsupported message type");
                    }
                }
                true
            }
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

// Handles CONNECTION parsing
fn parse_connect_message(buffer: &[u8]) -> Result<(), String> {
    // Checks minimum length for CONNECT message
    if buffer.len() < 14 {
        return Err("Message too short to be a valid CONNECT message".to_string());
    }

    // Parsing of protocol name length
    // The protocol name length is stored in the third and fourth bytes of the buffer (buffer[2] and buffer[3]).
    // These two bytes represent a 16-bit integer in big-endian format.

    // The following takes the first 8 bits of the buffer (buffer[2]) and shifts them to the left by 8 bits
    // Example: '0000 0000 0000 0000 0000 0000 0000 0100' << 8 = '0000 0000 0000 0100 0000 0000 0000 0000'
    // Then takes the next 8 bits of the buffer and adds them to the previous result
    // Example: '0000 0000 0000 0100 0000 0000 0000 0000' | '0000 0000 0000 0000 0000 0000 0000 0100' = '0000 0000 0000 0100 0000 0000 0000 0100'
    // This gets us the length of the protocol name and converts it to usize
    let protocol_name_len = ((buffer[2] as usize) << 8) | buffer[3] as usize;

    // Get's the protocol name from the buffer[start..end]
    let protocol_name = &buffer[4..4 + protocol_name_len];

    // Checks if protocol name is valid
    if protocol_name != b"MQTT" {
        return Err("Invalid protocol name".to_string());
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6942")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Connection failed: {}", e),
        }
    }
    Ok(())
}