use std::{io::Read, net::TcpStream, sync::MutexGuard};
use crate::mqtt::{broker_state::BrokerState, create_client::create_client, message_sender::send_response, message_type::MessageType};

// Function to handle incoming connections
pub fn connect(stream: &mut TcpStream, buffer: &mut [u8], thread_id: f64, broker_state: MutexGuard<'_, BrokerState>) {
    match stream.read(buffer) {
        Ok(_size) => {
            match MessageType::from_u8(buffer[0]) {
                // Handle Connect message
                Some(MessageType::Connect) => {
                    println!("CONNECT message received");
                    handle_connect(stream,&buffer, thread_id, broker_state);
                }
                _ => {
                    println!("First command must be connect");
                    return;
                }
            }
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }
    }
}

pub fn handle_connect(stream: &mut TcpStream, buffer: &[u8], thread_id: f64, broker_state: MutexGuard<'_, BrokerState>) {
    // Remaining code...
    create_client(&buffer, thread_id, broker_state, stream);
    send_connack(stream);
}

// Function to send Connack response
fn send_connack(stream: &mut TcpStream) {
    let mut response: [u8; 4] = [0; 4];
    response[0] = MessageType::Connack.to_u8();
    response[1] = 2;
    send_response(stream, &response);
}