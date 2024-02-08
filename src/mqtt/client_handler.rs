use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::mqtt::message_handlers::connect_handler::connect;
use crate::mqtt::message_handlers::ping_handler::ping_resp;
use crate::mqtt::message_handlers::publish_handler::{handle_publish, process_publish};
use crate::mqtt::message_handlers::subscribe_handler::handle_subscribe;
use crate::mqtt::message_handlers::unsubscribe_handle::handle_unsubscribe;
use crate::mqtt::message_sender::{get_packet_identifier_to_u16, send_response_packet};
use crate::mqtt::message_type::MessageType;
use time::{OffsetDateTime, PrimitiveDateTime};

use super::broker_state::{BrokerState, MessageState};

// Handles client connection
pub fn handle_client(mut stream: TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64) {
    println!("{}", thread_id);
    let mut buffer = [0; 2024];
    let mut first_stream = stream.try_clone().expect("Cannot clone stream");
    
    let current_broker_state = arc_broker_state.lock().unwrap();
    
    connect(&mut first_stream, &mut buffer, thread_id, current_broker_state);
    

    let mut current_broker_state = arc_broker_state.lock().unwrap();

    let current_client = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id ).unwrap().1;
    first_stream.set_read_timeout(Some(Duration::from_secs((current_client.keep_alive_seconds as f64 * 1.5) as u64))).unwrap();

    //Force unlock of broker state.
    std::mem::drop(current_broker_state);
    // Reads data from stream until connection is closed
    'tcpReader: while match first_stream.read(&mut buffer) {
        Ok(size) => {
            let mut current_broker_state = arc_broker_state.lock().unwrap();
            let current_client = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id ).unwrap().1;

            if size == 0 {
                println!("No data received, closing connection");
                break 'tcpReader;
            }

            let now = OffsetDateTime::now_utc();
            current_client.last_connection = PrimitiveDateTime::new(now.date(), now.time());

            match MessageType::from_u8(buffer[0]) {
                // Publish
                Some(MessageType::Publish) =>{
                    println!("PUBLISH message received");
                    handle_publish(&mut stream, &buffer, current_broker_state);
                }
                // Puback
                Some(MessageType::Puback) =>{
                    println!("PUBACK message received");
                    // Received puback from client/subscriber (QoS 1) - updates state to message acknowledged
                    let packet_identifier = get_packet_identifier_to_u16(&buffer, 2);
                    current_client.update_message_state(packet_identifier, MessageState::MessageAcknowledged);
                }
                // Pubrec
                Some(MessageType::Pubrec) =>{
                    println!("PUBREC message received");
                    // Received pubrec from client/subscriber (QoS 2) - updates state to message received
                    let packet_identifier = get_packet_identifier_to_u16(&buffer, 2);
                    current_client.update_message_state(packet_identifier, MessageState::MessageReceived);
                    send_response_packet(&mut stream, MessageType::Pubrel, packet_identifier);
                }
                // Pubrel
                Some(MessageType::Pubrel) =>{
                    println!("PUBREL message received");
                    // Received pubrel from publisher (QoS 2) - updates state to publish released
                    let packet_identifier = get_packet_identifier_to_u16(&buffer, 2);
                    current_client.update_message_state(packet_identifier, MessageState::PublishReleased);
                    send_response_packet(&mut stream, MessageType::Pubcomp, packet_identifier);
                }
                // Pubcomp
                Some(MessageType::Pubcomp) =>{
                    println!("PUBCOMP message received");
                    // Received pubcomp from client/subscriber (QoS 2) - updates state to message completed
                    let packet_identifier = get_packet_identifier_to_u16(&buffer, 2);
                    current_client.update_message_state(packet_identifier, MessageState::MessageCompleted);
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
            
        current_client.cancellation_requested = true;
        let current_client = current_client.clone();
        process_publish(&mut current_broker_state1, &current_client.will_topic, &current_client.will_text, current_client.will_qos, MessageState::None, 44);

    }
}


