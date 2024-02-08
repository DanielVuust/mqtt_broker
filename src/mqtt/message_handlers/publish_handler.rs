use std::{net::TcpStream, sync::MutexGuard};

use crate::mqtt::{broker_state::{BrokerState, SubscriptionMessage}, message_handlers::message_reader::{read_package_length, read_utf_8_string_with_end_index, read_utf_8_string_with_length_bytes}, message_type::MessageType};
use crate::mqtt::broker_state::MessageState;
use crate::mqtt::message_sender::{send_response_packet, get_packet_identifier_to_u16};


pub fn handle_publish(stream: &mut TcpStream, buffer: &[u8], mut broker_state: MutexGuard<'_, BrokerState>){
    let ( topic, message, qos, packet_identifier) = read_publish_bytes(buffer);
    
    match qos{
            1 => {
                process_publish(&mut broker_state, &topic, &message, qos, MessageState::PublishAcknowledged, packet_identifier);
                send_response_packet(stream, MessageType::Puback, packet_identifier);
            },
            2 => {
                process_publish(&mut broker_state, &topic, &message, qos, MessageState::PublishReceived, packet_identifier);
                send_response_packet(stream, MessageType::Pubrec, packet_identifier);
            },
            _ => {
                process_publish(&mut broker_state, &topic, &message, qos, MessageState::None, packet_identifier);
            }
        }
}


fn read_publish_bytes(buffer: &[u8]) -> (String, String, u8, u16){
    let mut reader_index = 0;

    println!("{:?}", buffer);

    // Get's the qos from the first byte in the buffer
    let qos = (buffer[reader_index] & 0x06) >> 1;
    
    reader_index += 1;

    let package_length: usize;
    (package_length, reader_index) = read_package_length(buffer, reader_index);
    
    let topic: String;
    // Reads the topics from buffer
    (topic, reader_index) = read_utf_8_string_with_length_bytes(buffer, reader_index);
    
    let mut packet_identifier: u16 = 0;

    if qos > 0 {
        packet_identifier = get_packet_identifier_to_u16(buffer, reader_index);

        reader_index += 2; // Skips the packet identifier
    }

    let message: String;
    // Reads the message from buffer
    (message, reader_index) = read_utf_8_string_with_end_index(buffer, reader_index, package_length-1);
    let _ = reader_index;
    (topic, message, qos, packet_identifier)
}

// Function to process the publish message
pub fn process_publish(broker_state: &mut MutexGuard<'_, BrokerState>, topic: &String, message: &String, qos: u8, message_state: MessageState, packet_identifier: u16) {
    //Loops though all connected clients and checks if they have any subscriptions that matches the topic,
    //if so the message will be push to the subscription message list 
    for client in broker_state.clients.iter_mut() {
        for subscription in client.subscriptions.iter_mut() {
            if subscription.topic_title == *topic {
                subscription.messages.push(SubscriptionMessage::new(
                    message.clone(),
                    qos,
                    message_state.clone(),
                    packet_identifier
                ));
            }
        }
    }
}