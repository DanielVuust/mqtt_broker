use std::{net::TcpStream, ops::Sub, sync::MutexGuard};

use crate::mqtt::{broker, broker_state::{BrokerState, SubscriptionMessage}, message_handlers::message_reader::{read_package_length, read_uft_8_string_with_end_index}, message_sender::send_response, message_type::MessageType};

use super::message_reader::read_uft_8_string_with_length_bytes;


pub fn handle_publish(stream: &mut TcpStream, buffer: &[u8], thread_id: f64, mut broker_state: MutexGuard<'_, BrokerState>){
    
    
    let topic: String;
    let message: String;

    (topic, message) = read_publish_bytes(buffer);

    //Loops though all connected clients and checks if they have any subscriptions that matches the topic,
    //if so the message will be push to the subscription message list 
    for i in (*broker_state).clients.iter_mut().enumerate(){
        for j in i.1.subscriptions.iter_mut().enumerate() {
            if j.1.topic_title == topic{
                j.1.messages.push(SubscriptionMessage::new(message.to_string()));
            }
        }
    }

    // send_puback_response(stream);
}


fn read_publish_bytes(buffer: &[u8]) -> (String, String){
    let mut reader_index = 1;

    println!("{:?}", buffer);
    
    let package_length: usize;
    (package_length, reader_index) = read_package_length(buffer, reader_index);
    
    let topic: String;
    (topic, reader_index) = read_uft_8_string_with_length_bytes(buffer, reader_index);
    
    let message: String;
    (message, reader_index) = read_uft_8_string_with_end_index(buffer, reader_index, package_length-1);

    (topic, message)
}

fn send_puback_response(stream: &mut TcpStream,){

    let mut response: [u8; 2] = [0; 2];
    response[0] = MessageType::Puback.to_u8();
    response[1] = 0;

    send_response(stream, &response);
}