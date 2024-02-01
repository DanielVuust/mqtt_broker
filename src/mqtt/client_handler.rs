use std::borrow::Borrow;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::mqtt::broker_state::Subscription;
use crate::mqtt::message_handlers::connect_handler::handle_connect;
use crate::mqtt::message_handlers::ping_handler::ping_resp;
use crate::mqtt::message_handlers::subscribe_handler::handle_subscribe;
use crate::mqtt::message_sender::{ send_response};
use crate::mqtt::message_type::MessageType;
use crate::mqtt::utils::get_length;

use super::broker_state::BrokerState;

// Handles client connection
pub fn handle_client(mut stream: TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64) {
    println!("{}", thread_id);
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

    // Reads data from stream until connection is closed
    while match stream.read(&mut buffer) {
        
        Ok(size) => {
            let mut current_broker_state = arc_broker_state.lock().unwrap();
            
            println!("{:?}", current_broker_state);
            if size == 0 {
                // No data received, closing connection
                false
            } else {

                last_communication = SystemTime::now();
                match MessageType::from_u8(buffer[0]) {
                    // Connect
                    Some(MessageType::Connect) => {
                        println!("CONNECT message received");
                        (client_id, will_topic, will_text, will_retain, will_qos, clean_session, keep_alive_secounds) = handle_connect(&buffer);
                        
                        let t = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id );
                        t.unwrap().1.client_id = client_id;

                        let mut response: [u8; 4] = [0; 4];
                        response[0] = MessageType::Connack.to_u8();
                        response[1] = 2;
                        send_response(&mut stream, &response);
                        
                        
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
                        
                        handle_subscribe(&mut stream, &buffer, thread_id, current_broker_state);
                       
                    }
                    // Unsubscribe
                    Some(MessageType::Unsubscribe) =>{
                        println!("Unsubscribe message received");
                    }
                    // Pingreq
                    Some(MessageType::Pingreq) =>{
                        println!("PINGREQ message received");
                        ping_resp(&mut stream, MessageType::Pingresp);
                    }
                    // Disconnect
                    Some(MessageType::Disconnect) => {
                        println!("DISCONNECT message received");
                        return;
                    }
                    // Invalid or unsupported
                    _ => {
                        println!("Invalid or unsupported message type");
                    }
                }
                println!("{:?}", MessageType::from_u8(buffer[0]));

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
