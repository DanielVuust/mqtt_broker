use std::{net::TcpStream, sync::MutexGuard};

use crate::mqtt::{broker_state::{BrokerState, Subscription}, message_sender::send_response, message_type::MessageType};

pub fn handle_subscribe(stream: &mut TcpStream, buffer: &[u8], thread_id: f64, broker_state: MutexGuard<'_, BrokerState>){
    // Extract the topic name and QoS level
    let (topic, requested_qos) = get_topic_name_and_qos(buffer, 4);
    println!("Topic: {:?}, Requested QoS: {}", topic, requested_qos);

    // Change broker state to include the new subscription with its requested QoS
    add_topic_to_broker_state(topic.clone(), thread_id, requested_qos, broker_state);

    // Create a SUBACK response packet.
    let mut response: [u8; 5] = [0; 5];
    response[0] = MessageType::Suback.to_u8();
    response[1] = 0x03; // Remaining length
    response[2] = buffer[2]; // Packet identifier MSB
    response[3] = buffer[3]; // Packet identifier LSB
    response[4] = requested_qos; // QoS level

    // Send the SUBACK response to the client.
    send_response(stream, &response);
}

fn get_topic_name_and_qos(buffer: &[u8], start_index: usize) -> (String, u8) {
    let topic_length = (buffer[start_index] as usize) << 8 | (buffer[start_index + 1] as usize);
    let mut current_index = start_index + 2;

    let topic = String::from_utf8(buffer[current_index..current_index + topic_length].to_vec()).unwrap();
    current_index += topic_length;

    let qos = buffer[current_index];

    (topic, qos)
}

fn add_topic_to_broker_state(topic: String, thread_id: f64, qos: u8, mut broker_state: MutexGuard<'_, BrokerState>) {
    if let Some(client) = (*broker_state).clients.iter_mut().find(|client| client.thread_id == thread_id) {
        client.subscriptions.push(Subscription::new(topic, qos));
    }
}