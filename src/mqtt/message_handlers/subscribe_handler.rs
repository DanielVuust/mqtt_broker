use std::{net::TcpStream, sync::MutexGuard};

use crate::mqtt::{broker_state::{BrokerState, Subscription}, message_sender::send_response, message_type::MessageType};



pub fn handle_subscribe(stream: &mut TcpStream, buffer: &[u8], thread_id: f64, broker_state: MutexGuard<'_, BrokerState>){
    let _package_identifier = buffer[2] as usize * 256 + buffer[3] as usize;

    //Get topic names from buffer:
    let topic = get_topic_name(buffer, 4);
    println!("{:?}", topic);

    //Change broker state
    add_topic_to_broker_state(topic, thread_id, broker_state);

    //Send response 
    let mut response: [u8; 5] = [0; 5];
    response[0] = MessageType::Suback.to_u8();
    response[1] = 0x03;
    response[2] = buffer[2];
    response[3] = buffer[3];
    response[4] = 0x00;
    println!("{:?}", response);
    send_response(stream, &response);
}

fn get_topic_name(buffer: &[u8], mut current_buffer_index: usize) -> String {

    let topic_length = buffer[current_buffer_index] as usize * 256 as usize + buffer[current_buffer_index + 1] as usize;
    current_buffer_index += 2;

    let mut topic = "".to_string();
    for index in current_buffer_index..current_buffer_index+topic_length{
        topic.push(buffer[index] as char);
    }
    current_buffer_index += topic_length;
    let _topic_qos: u8 = buffer[current_buffer_index];
    current_buffer_index += 1;

    let _ = current_buffer_index;
    
    topic
}

fn add_topic_to_broker_state(topic: String, thread_id: f64, mut broker_state: MutexGuard<'_, BrokerState>){
    let t = (*broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id );
    t.unwrap().1.subscriptions.push(Subscription::new(topic));
}