use std::{net::{TcpListener, TcpStream}, string};

use crate::mqtt::{message_sender::{send_response}, message_type::MessageType};



pub fn handle_subscribe(stream: &mut TcpStream, buffer: &[u8]){
    let package_identifier = buffer[2] as usize * 256 + buffer[3] as usize;

    let topic = get_topic_name(buffer, 4);
    println!("{:?}", topic);

    let mut response: [u8; 6] = [0; 6];
    response[0] = MessageType::Suback.to_u8();
    response[1] = 0;
    response[2] = buffer[2];
    response[4] = buffer[3];
    response[5] = 0x00;

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
    let topic_qos: u8 = buffer[current_buffer_index];
    current_buffer_index += 1;

    topic
}