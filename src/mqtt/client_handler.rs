use std::io::Read;
use std::net::TcpStream;
use std::time::SystemTime;
use crate::mqtt::message_handlers::connect_handler::handle_connect;
use crate::mqtt::message_handlers::ping_handler::ping_resp;
use crate::mqtt::message_handlers::subscribe_handler::handle_subscribe;
use crate::mqtt::message_type::MessageType;
use crate::mqtt::message_sender::send_answer;

use super::message_protocol_parser::parse_connect_message;

// Handles client connection
pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let mut client_id: String = "".to_string();    
    let mut will_topic: String  = "".to_string();
    let mut will_text: String  = "".to_string();
    let mut will_retain: bool;
    //Chmut ange to two;bits 
    let mut will_qos: u8;
    let mut clean_session: bool;
    let mut keep_alive_secounds: usize;
    let mut last_communication: SystemTime;

    let mut subscribed_topics: Vec<String> = vec![];

    // Reads data from stream until connection is closed
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // No data received, closing connection
                false
            } else {
                // Uses bit-shift operation to move first 4 bits to the right of the byte and converts it to u8 
                // Example: '0001 1010' >> 4 = '0000 0001' = 1u8
                let msg_type = buffer[0] >> 4;

                last_communication = SystemTime::now();
                match MessageType::from_u8(msg_type) {
                    // Connect
                    Some(MessageType::Connect) => {
                        match parse_connect_message(&buffer[..size]) {
                            Ok(_) => {
                                println!("CONNECT message received");
                                (client_id, will_topic, will_text, will_retain, will_qos, clean_session, keep_alive_secounds) = handle_connect(&buffer);
                                send_answer(&mut stream, MessageType::Connack);
                            },
                            Err(e) => println!("Error parsing CONNECT message: {}", e),
                        }
                    }
                    // Publish
                    Some(MessageType::Publish) =>{
                        println!("PUBLISH message received");
                    }
                    // Puback
                    Some(MessageType::Puback) =>{
                        println!("PUBACK message received");
                    }
                    // Pubrec
                    Some(MessageType::Pubrec) =>{
                        println!("PUBREC message received");
                    }
                    // Pubrel
                    Some(MessageType::Pubrel) =>{
                        println!("PUBREL message received");
                    }
                    // Pubcomp
                    Some(MessageType::Pubcomp) =>{
                        println!("PUBCOMP message received");
                    }
                    // Subscribe
                    Some(MessageType::Subscribe) =>{
                        println!("SUBSCRIBE message received");
                        println!("{:?}", buffer);
                        let topic = handle_subscribe(&buffer, 5);
                        subscribed_topics.push(topic);
                        println!("{:?}", subscribed_topics);
                        send_answer(&mut stream, MessageType::Suback);
                           
                    }
                    // Unsubscribe
                    Some(MessageType::Unsubscribe) =>{
                        println!("Unsubscribe message received");
                    }
                    // Pingreq
                    Some(MessageType::Pingreq) =>{
                        println!("PINGREQ message received");
                        ping_resp(&mut stream, MessageType::Pingreq);
                    }
                    // Disconnect
                    Some(MessageType::Disconnect) => {
                        println!("DISCONNECT message received");
                        return;
                    }
                    // Invalid or unsupported
                    _ => {
                        println!("Invalid or unsupported message type");
                        return;
                    }
                }
                buffer = [0; 2048];
                true
            }
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}
