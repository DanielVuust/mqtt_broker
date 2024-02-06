use std::{net::TcpStream, ops::Sub, sync::{Arc, Mutex, MutexGuard}};
use crate::mqtt::{broker_state::{BrokerState, Client, MessageState}, message_sender::{get_packet_identifier_to_u16, send_response_packet}, message_type::MessageType};

// Handle PUBREL message
pub fn handle_pubrel(stream: &mut TcpStream, buffer: &[u8], client: &mut Client) {
    let packet_identifier = get_packet_identifier_to_u16(buffer, 2);

    // Finds the message in the client and updates the state to Released
    if client.subscriptions.len() > 0 {
        for subscription in &mut client.subscriptions {
            for message in &mut subscription.messages {
                if message.packet_identifier == packet_identifier {
                    message.message_state = MessageState::Released;
                }
            }
        }
    }

    // Sends the PUBCOMP message
    send_response_packet(stream, MessageType::Pubcomp, packet_identifier);
    println!("PUBCOMP message sent")
}