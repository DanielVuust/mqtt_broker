use std::net::TcpStream;

use crate::mqtt::{broker_state::Client, message_sender::send_response, message_type::MessageType};



pub fn handle_unsubscribe(stream: &mut TcpStream, buffer: &[u8], client: &mut Client ){

    let (topic_to_unsunscribe_from, package_identifier) = read_unsubscribe_buffer(buffer);

    unsubscribe_from_topics(topic_to_unsunscribe_from, client);

    send_unsuback(stream, package_identifier);
}

fn read_unsubscribe_buffer(buffer: &[u8]) -> (Vec<String>, u16) {
    let mut topic_to_unsunscribe_from: Vec<String> = vec![];

    let mut reader_index = 0;
    println!("{}", buffer[reader_index]);
    // if buffer[reader_index] !=  162{
    //     panic!("Unsubscribe command is not the correct control package");
    // }
    reader_index += 1;
    
    let remaining_length = buffer[reader_index];

    reader_index += 1;

    let package_identifier: u16 = buffer[reader_index] as u16 * 256 + buffer[reader_index + 1] as u16;

    reader_index += 2;


    while reader_index < (remaining_length + 2) as usize{
        let current_topic_length = get_length(&buffer, reader_index);
        println!("current_topic_length: {}", current_topic_length);
        reader_index += 2;
        let mut topic = "".to_string();
        for index in reader_index..reader_index+current_topic_length{
            topic.push(buffer[index] as char);
        }
        topic_to_unsunscribe_from.append(&mut vec![topic]);
        reader_index += current_topic_length;
    }

    (topic_to_unsunscribe_from, package_identifier)
}
fn unsubscribe_from_topics(topics_to_unsunscribe_from: Vec<String>, client: &mut Client ){
    for topic in topics_to_unsunscribe_from{
        //Handle "It MUST complete the delivery of any QoS 1 or QoS 2 messages which it has started to send to the Clien" from docs
        let index = client.subscriptions.iter().position(|r| r.topic_title == topic).unwrap();
        client.subscriptions.remove(index);
    }
}
// Get's length of message length from index
fn get_length(buffer: &[u8], index: usize) -> usize {
    buffer[index] as usize * 256 as usize + buffer[index + 1] as usize
}
fn send_unsuback(stream: &mut TcpStream, _package_identifier: u16){
    // Create a unsuback response packet.
    let mut response: [u8; 4] = [0; 4];
    response[0] = MessageType::Unsuback.to_u8();
    response[1] = 0x02; // Remaining length
    response[2] = (_package_identifier / 256) as u8; // Packet identifier MSB
    response[3] = (_package_identifier % 256) as u8; // Packet identifier LSB

    send_response(stream, &response);
}