use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;
use crate::mqtt::message_handlers::connect_handler::connect;
use crate::mqtt::message_handlers::ping_handler::ping_resp;
use crate::mqtt::message_handlers::publish_handler::handle_publish;
use crate::mqtt::message_handlers::subscribe_handler::handle_subscribe;
use crate::mqtt::message_handlers::unsubscribe_handle::handle_unsubscribe;
use crate::mqtt::message_sender::{get_packet_identifier_to_u16, send_response, send_response_packet};
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

    // Reads data from stream until connection is closed
    while match first_stream.read(&mut buffer) {
        
        Ok(size) => {
            let mut current_broker_state = arc_broker_state.lock().unwrap();
            let current_client = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id ).unwrap().1;
            let packet_identifier: u16 = get_packet_identifier_to_u16(&buffer, 2);

            if size == 0 {
                println!("No data received, closing connection");
                return;
            }

            let now = OffsetDateTime::now_utc();
            current_client.last_connection = PrimitiveDateTime::new(now.date(), now.time());

            match MessageType::from_u8(buffer[0]) {
                // Publish
                Some(MessageType::Publish) =>{
                    println!("PUBLISH message received");
                    handle_publish(packet_identifier, &mut stream, &buffer, current_broker_state);
                }
                // Puback
                Some(MessageType::Puback) =>{
                    println!("PUBACK message received");
                    // Received puback from client/subscriber (QoS 1) - updates state to message acknowledged
                    current_client.update_message_state(packet_identifier, MessageState::MessageAcknowledged);
                }
                // Pubrec
                Some(MessageType::Pubrec) =>{
                    println!("PUBREC message received");
                    // Received pubrec from client/subscriber (QoS 2) - updates state to message received
                    current_client.update_message_state(packet_identifier, MessageState::MessageReceived);
                }
                // Pubrel
                Some(MessageType::Pubrel) =>{
                    println!("PUBREL message received");
                    // Received pubrel from publisher (QoS 2) - updates state to publish released
                    current_client.update_message_state(packet_identifier, MessageState::PublishReleased);
                    send_response_packet(&mut stream, MessageType::Pubcomp, packet_identifier);
                }
                // Pubcomp
                Some(MessageType::Pubcomp) =>{
                    println!("PUBCOMP message received");
                    // Received pubcomp from client/subscriber (QoS 2) - updates state to message completed
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
                    return;
                }
                // Invalid or unsupported
                _ => {
                    println!("Invalid or unsupported message type");
                    return;
                }
            }

            //println!("{:?}", MessageType::from_u8(buffer[0]));

            buffer = [0; 2024];
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn handle_second_stream( stream: &mut TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64,){
    loop{
        sleep(Duration::from_millis(750));
        let mut current_broker_state = arc_broker_state.lock().unwrap();
        let client = current_broker_state.clients.iter_mut()
            .find(|client| client.thread_id == thread_id)
            .unwrap();

        let mut completed_messages = Vec::new();

        for subscription in &mut client.subscriptions {
            for message in &mut subscription.messages {
                // Resends message if no response packet from client (QoS 1 and 2)
                if matches!(message.message_state, MessageState::PublishSent) && 
                message.last_updated + time::Duration::seconds(10) < OffsetDateTime::now_utc() ||
                matches!(message.message_state, MessageState::MessageReceived) &&
                message.last_updated + time::Duration::seconds(10) < OffsetDateTime::now_utc()
                {
                    send_publish_message(stream, subscription.topic_title.to_string(), message.message.to_string(), true);
                    message.add_retry();
                }

                // Sends new message
                if matches!(message.message_state, MessageState::PublishAcknowledged) || 
                matches!(message.message_state, MessageState::PublishReleased) || 
                matches!(message.message_state, MessageState::None) 
                {
                    if message.qos > 0 {
                        message.update_state(MessageState::PublishSent);
                    }
                    send_publish_message(stream, subscription.topic_title.to_string(), message.message.to_string(), false);
                }

                // Removes message from subscription message list if completed
                if matches!(message.message_state, MessageState::MessageCompleted) ||
                matches!(message.message_state, MessageState::MessageAcknowledged) ||
                matches!(message.message_state, MessageState::None) 
                {
                    completed_messages.push(message.packet_identifier);
                }
            }
        }

        // Prints all messages in subscription message list
        // TODO: Remove this when done testing
        println!("Messages in subscription list:");
        for subscription in &mut client.subscriptions {
            for message in &mut subscription.messages {
                println!("[]: {:?}", message);
            }
        }

        // Removes completed messages from subscription message list
        for subscription in &mut client.subscriptions {
            subscription.messages.retain(|message| !completed_messages.contains(&message.packet_identifier));
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

fn send_publish_message(stream: &mut TcpStream, topic_name: String, message: String, is_retry: bool){
    let mut response: Vec<u8> = [].to_vec();

    let mut message_type = MessageType::Publish.to_u8();

    if is_retry {
        message_type |= 0b00001000; // Sets the DUP flag to 1 - if message is a retry
    }

    response.push(message_type);
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