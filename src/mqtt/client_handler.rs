use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;
use crate::mqtt::message_handlers::connect_handler::connect;
use crate::mqtt::message_handlers::ping_handler::ping_resp;
use crate::mqtt::message_handlers::puback_handler::handle_puback;
use crate::mqtt::message_handlers::pubcomp_handler::handle_pubcomp;
use crate::mqtt::message_handlers::publish_handler::{handle_publish, process_publish};
use crate::mqtt::message_handlers::pubrec_handler::handle_pubrec;
use crate::mqtt::message_handlers::pubrel_handler::handle_pubrel;
use crate::mqtt::message_handlers::subscribe_handler::handle_subscribe;
use crate::mqtt::message_handlers::unsubscribe_handle::handle_unsubscribe;
use crate::mqtt::message_sender::send_response;
use crate::mqtt::message_type::MessageType;
use time::{OffsetDateTime, PrimitiveDateTime};

use super::broker_state::{BrokerState, MessageState};

// Handles client connection
pub fn handle_client(mut stream: TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64) {
    println!("{}", thread_id);
    let mut buffer = [0; 2024];
    let mut first_stream = stream.try_clone().expect("Cannot clone stream");
    let mut second_stream = stream.try_clone().expect("Cannot clone stream");
    
    let current_broker_state = arc_broker_state.lock().unwrap();

    connect(&mut first_stream, &mut buffer, thread_id, current_broker_state);

    let arc2 = Arc::clone(&arc_broker_state);
    thread::spawn(move || {
        handle_second_stream(&mut second_stream, arc2, thread_id);
    });
    
    println!("continue");

    //Delete this
    first_stream.set_read_timeout(Some(Duration::from_secs(10)));

    // Reads data from stream until connection is closed
    'tcpReader: while match first_stream.read(&mut buffer) {
        
        Ok(size) => {
            let mut current_broker_state = arc_broker_state.lock().unwrap();
            let current_client = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id ).unwrap().1;

            if size == 0 {
                // No data received, closing connection
                false
            } 
            else {
                let now = OffsetDateTime::now_utc();
                current_client.last_connection = PrimitiveDateTime::new(now.date(), now.time());
                match MessageType::from_u8(buffer[0]) {
                    // Publish
                    Some(MessageType::Publish) =>{
                        println!("PUBLISH message received");
                        handle_publish(&mut stream, &buffer, thread_id, current_broker_state);
                    }
                    // Puback
                    Some(MessageType::Puback) =>{
                        println!("PUBACK message received");
                        handle_puback(&mut stream, &buffer, current_client);
                    }
                    // Pubrec
                    Some(MessageType::Pubrec) =>{
                        println!("PUBREC message received");
                        handle_pubrec(&mut stream, &buffer, current_client);
                    }
                    // Pubrel
                    Some(MessageType::Pubrel) =>{
                        println!("PUBREL message received");
                        handle_pubrel(&mut stream, &buffer, current_client);
                    }
                    // Pubcomp
                    Some(MessageType::Pubcomp) =>{
                        println!("PUBCOMP message received");
                        handle_pubcomp(&mut stream, &buffer, current_client);
                    }
                    // Subscribe
                    Some(MessageType::Subscribe) =>{
                        println!("SUBSCRIBE message received");
                        handle_subscribe(&mut stream, &buffer, thread_id, current_broker_state);
                    }
                    // Unsubscribe
                    Some(MessageType::Unsubscribe) =>{
                        println!("Unsubscribe message received");
                        handle_unsubscribe(&mut stream, &buffer, current_client);
                    }
                    // Pingreq
                    Some(MessageType::Pingreq) =>{
                        println!("PINGREQ message received");
                        ping_resp(&mut stream, MessageType::Pingresp);
                    }
                    // Disconnect
                    Some(MessageType::Disconnect) => {
                        println!("DISCONNECT message received");
                        break 'tcpReader;
                    }
                    // Invalid or unsupported
                    _ => {
                        println!("Invalid or unsupported message type");
                        break 'tcpReader;
                    }
                }
                //println!("{:?}", MessageType::from_u8(buffer[0]));

                buffer = [0; 2024];
                true
            }
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false;
            break 'tcpReader;
        }
    }
    {}
    {
    // Store will message to subscriber.
    let mut current_broker_state1 = arc_broker_state.lock().unwrap();
    let current_client = current_broker_state1.clients.iter_mut().find(|client| client.thread_id == thread_id).unwrap();
    
    let current_client = current_client.clone();
    process_publish(&mut current_broker_state1, &current_client.will_topic, &current_client.will_text, current_client.will_qos, MessageState::None, 44);

    }
}


fn handle_second_stream( stream: &mut TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64,){
    
    loop{
        sleep(Duration::from_millis(750));
        let mut current_broker_state = arc_broker_state.lock().unwrap();
        let client = current_broker_state.clients.iter_mut()
            .find(|client| client.thread_id == thread_id)
            .unwrap();

        for subscription in &mut client.subscriptions {
            for message in &mut subscription.messages {
                if matches!(message.message_state, MessageState::PublishAcknowledged) || 
                matches!(message.message_state, MessageState::PublishReleased) || 
                matches!(message.message_state, MessageState::None) 
                {
                    send_publish_message(stream, subscription.topic_title.to_string(), message.message.to_string());
                    message.update_state(MessageState::PublishSent);
                }
            }
        }
        
        let now = OffsetDateTime::now_utc();

        //TODO check keep alive from client
        if client.last_connection + time::Duration::seconds((60 as f64 * 1.5) as i64) < PrimitiveDateTime::new(now.date(), now.time()) {
            println!("Killing connection due to no ping from client");
            client.cancellation_requested = true;
            return;
        }
    }
}

fn send_publish_message(stream: &mut TcpStream, topic_name: String, message: String){

    let mut response: Vec<u8> = [].to_vec();
    response.push(MessageType::Publish.to_u8());
    response.push(0);
    response.push((topic_name.len() / 256) as u8);
    response.push((topic_name.len() % 256) as u8);
    
    for i in topic_name.chars(){
        response.push(i as u8);
        
    }
    
    for i in message.chars(){
        response.push(i as u8);

    }
    
    response[1] = response.len() as u8 - 2;

    println!("{:?}", response);

    send_response(stream, &response);
}