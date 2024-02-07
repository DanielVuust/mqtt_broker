use std::net::TcpStream;
use crate::mqtt::{broker_state::{Client, MessageState}, message_sender::{get_packet_identifier_to_u16, send_response_packet}, message_type::MessageType};

// Handle PUBACK message
pub fn handle_puback(stream: &mut TcpStream, buffer: &[u8], client: &mut Client) {
    let packet_identifier = get_packet_identifier_to_u16(buffer, 2);

    // Finds the message in the client and updates the state to Acknowledged
    //if client.subscriptions.len() > 0 {
    //    for subscription in &mut client.subscriptions {
    //        for message in &mut subscription.messages {
    //            if message.packet_identifier == packet_identifier {
    //                message.update_state(MessageState::SubscriptionAcknowledged);
    //            }
    //        }
    //    }
    //}

    // Deletes the message from the client
    if client.subscriptions.len() > 0 {
        for subscription in &mut client.subscriptions {
            subscription.messages.retain(|message| message.packet_identifier != packet_identifier);
        }
    }
}