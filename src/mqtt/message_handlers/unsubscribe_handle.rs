use std::net::TcpStream;

use crate::mqtt::broker_state::Client;



pub fn handle_unsubscribe(stream: &mut TcpStream, buffer: &[u8], client: &mut Client ){

    let (topic_to_unsunscribe_from, package_identifier) = read_unsubscribe_buffer(buffer);

    unsubscribe_from_topics(topic_to_unsunscribe_from, client)

    send_unsuback(stream, package_identifier);
}

fn read_unsubscribe_buffer(buffer: &[u8]) -> (Vec<String>, String) {
    let mut topic_to_unsunscribe_from: Vec<String> = vec![];

    let mut reader_index = 0;
    println!("{}", buffer[reader_index]);
    // if buffer[reader_index] !=  162{
    //     panic!("Unsubscribe command is not the correct control package");
    // }
    reader_index += 1;
    
    let remaining_length = buffer[reader_index];

    reader_index += 1;

    let package_identifier = buffer[reader_index];

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
fn send_unsuback(tream: &mut TcpStream, package_identifier: u16){

}