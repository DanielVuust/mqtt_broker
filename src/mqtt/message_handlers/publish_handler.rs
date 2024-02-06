use std::{net::TcpStream, ops::Sub, sync::MutexGuard};

use crate::mqtt::{broker, broker_state::{BrokerState, SubscriptionMessage}, message_handlers::message_reader::{read_package_length, read_uft_8_string_with_end_index}, message_sender::send_response, message_type::MessageType, utils::get_message_length};

use super::message_reader::read_uft_8_string_with_length_bytes;


pub fn handle_publish(stream: &mut TcpStream, buffer: &[u8], thread_id: f64, mut broker_state: MutexGuard<'_, BrokerState>){

    println!("Guard publish: {:?}", buffer);

    let message_length = get_length(&buffer, 2);

    println!("Message length: {:?}", message_length);

    let ( topic, message, qos) = read_publish_bytes(buffer);

    //Loops though all connected clients and checks if they have any subscriptions that matches the topic,
    //if so the message will be push to the subscription message list 

    for i in (*broker_state).clients.iter_mut().enumerate(){
        for j in i.1.subscriptions.iter_mut().enumerate() {
            if j.1.topic_title == topic{
                j.1.messages.push(SubscriptionMessage::new(
                    message.to_string(), 
                    qos,
                    false,
                    false
                ));
            }
        }
    }

    if qos == 1 {
        send_puback_response(stream, buffer[message_length + 4],  buffer[message_length + 5]);
    }
}


fn read_publish_bytes(buffer: &[u8]) -> (String, String, u8){
    let mut reader_index = 0;

    println!("{:?}", buffer);

    // Get's the qos from the first byte in the buffer
    let qos = (buffer[reader_index] & 0x06) >> 1;
    
    reader_index += 1;

    let package_length: usize;
    (package_length, reader_index) = read_package_length(buffer, reader_index);
    
    let topic: String;
    // Reads the topics from buffer
    (topic, reader_index) = read_uft_8_string_with_length_bytes(buffer, reader_index);
    
    // If qos is greater than 0, the package identifier will be read from the buffer
    if qos > 0 {
        reader_index += 2;
    }

    let message: String;
    // Reads the message from buffer
    (message, reader_index) = read_uft_8_string_with_end_index(buffer, reader_index, package_length-1);

    (topic, message, qos)
}

fn send_puback_response(stream: &mut TcpStream, msb: u8, lsb: u8){

    println!("Sending puback response");
    println!("MSB: {:?}", msb);
    println!("LSB: {:?}", lsb);

    let mut response: [u8; 4] = [0; 4];
    response[0] = MessageType::Puback.to_u8();
    response[1] = 0x02; // Remaining length
    response[2] = msb; // MSB
    response[3] = lsb; // LSB
    
    send_response(stream, &response);
}

// Get's length of message length from index
fn get_length(buffer: &[u8], index: usize) -> usize {
    buffer[index] as usize * 256 as usize + buffer[index + 1] as usize
}