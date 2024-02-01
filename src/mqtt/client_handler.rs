use std::io::{self, Read};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;
use crate::mqtt::message_handlers::connect_handler::handle_connect;
use crate::mqtt::message_type::MessageType;
use crate::mqtt::message_sender::send_answer;
use crate::mqtt::message_protocol_parser::parse_connect_message;
use crate::mqtt::utils::get_length;

// Handles client connection
pub fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    // Sets read timeout to 30 seconds - if no data is received within this time, the connection is closed
    stream.set_read_timeout(Some(Duration::from_secs(30))).expect("Failed to set read timeout");

    loop {
        // Reads the fixed header
        let mut fixed_header = [0; 5];
        let mut size = match stream.read(&mut fixed_header) {
            Ok(size) if size == 0 => break,  // No data received, closing connection
            Ok(size) => size,
            Err(e) => return Err(e),
        };

        // Gets remaining length of the message
        let remaining_length = get_length(&fixed_header, 1) as usize;
        let total_length = remaining_length + size;

        // Creates a buffer with the size of the message
        let mut buffer = vec![0; total_length];
        buffer[..size].copy_from_slice(&fixed_header[..size]);

        let mut read_attempts = 0;
        let max_read_attempts = 10;

        // Reads the rest of the packet
        while size < total_length {
            if read_attempts >= max_read_attempts {
                println!("Maximum read attempts reached, incomplete message received");
                break;
            }

            match stream.read(&mut buffer[size..]) {
                Ok(0) => break, // No data received, closing connection
                Ok(n) => size += n,
                Err(e) => return Err(e),
            }

            read_attempts += 1; // Increments read attempts - stops reading if max_read_attempts is reached
        }

        println!("Received message: {:?}", buffer.len());

        // Processes the message
        let msg_type = buffer[0] >> 4; // Gets the message type from the first 4 bits of the first byte
        handle_message(MessageType::from_u8(msg_type).unwrap(), &buffer, &mut stream);
    }

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}





// Handles message
fn handle_message(msg_type: MessageType, buffer: &[u8], stream: &mut TcpStream) {
    match msg_type {
        MessageType::Connect => {
            match parse_connect_message(&buffer) {
                Ok(_) => {
                    println!("CONNECT message received");
                    handle_connect(&buffer);
                    send_answer(stream, MessageType::Connack);
                },
                Err(e) => println!("Error parsing CONNECT message: {}", e),
            }
        },
        MessageType::Publish => {
            println!("PUBLISH message received");
        },
        MessageType::Puback => {
            println!("PUBACK message received");
        },
        MessageType::Pubrec => {
            println!("PUBREC message received");
        },
        MessageType::Pubrel => {
            println!("PUBREL message received");
        },
        MessageType::Pubcomp => {
            println!("PUBCOMP message received");
        },
        MessageType::Subscribe => {
            println!("SUBSCRIBE message received");
        },
        MessageType::Unsubscribe => {
            println!("UNSUBSCRIBE message received");
        },
        MessageType::Pingreq => {
            println!("PINGREQ message received");
        },
        MessageType::Disconnect => {
            println!("DISCONNECT message received");
        },
        _ => {
            println!("Invalid or unsupported message type");
        }
    }
}