use std::{net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::sleep, time::Duration};
use rand::Rng;
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::mqtt::{client_handler::handle_client, message_sender::send_response, message_type::MessageType};
use std::thread;

use super::broker_state::{self, BrokerState, MessageState};


pub fn start_broker() -> () {
    println!("Starting broker");
    let listener = TcpListener::bind("0.0.0.0:7878");
    let broker_state: Arc<Mutex<BrokerState>> = Arc::new(Mutex::new(broker_state::BrokerState::new()));
    let cloned_broker_state = Arc::clone(&broker_state);
    thread::spawn(move || {
        start_state_keeper_thread(cloned_broker_state)
    });
    for stream in listener.unwrap().incoming() {
        let cloned_broker_state = Arc::clone(&broker_state);
        thread::spawn(move || {
            let thread_id = create_thread_id();
            handle_client(stream.unwrap(), cloned_broker_state, thread_id);
            
        });
    }
}

fn create_thread_id() -> f64{

    //Generate random thread id.
    let mut rng = rand::thread_rng();
    let thread_id: f64 = rng.gen(); 

    return thread_id;
}

fn start_state_keeper_thread(broker_state:  Arc<Mutex<BrokerState>>){
    
        loop{
            sleep(Duration::from_millis(750));
            let mut current_broker_state = broker_state.lock().unwrap();
            for client in current_broker_state.clients.iter_mut(){
                
                if client.cancellation_requested {
                    println!("Removing disconnected client form state");
                    client.tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
                    break;
                }
        
                let mut completed_messages = Vec::new();
                let message_retry_timer = 10;
                let max_retry_count = 5;
        
                for subscription in &mut client.subscriptions {
                    for message in &mut subscription.messages {
                        // Sends new message
                        if matches!(message.message_state, MessageState::PublishAcknowledged) || 
                        matches!(message.message_state, MessageState::PublishReleased) || 
                        matches!(message.message_state, MessageState::None) 
                        {
                            // Updates subscription QoS level if publisher QoS level is higher
                            if message.pub_qos < subscription.sub_qos {
                                subscription.sub_qos = message.pub_qos;
                            }

                            send_publish_message(message.packet_identifier, &mut client.tcp_stream, subscription.topic_title.to_string(), message.message.to_string(), false, subscription.sub_qos);
        
                            if message.pub_qos > 0 {
                                // Updates message to completed if QoS level on subscription is 0
                                if subscription.sub_qos == 0 {
                                    message.update_state(MessageState::MessageCompleted);
                                }else {
                                    message.update_state(MessageState::PublishSent);
                                }
                            }
                        }
        
                        // Resends message if no response packet from client (QoS 1 and 2)
                        if matches!(message.message_state, MessageState::PublishSent) && 
                        message.last_updated + time::Duration::seconds(message_retry_timer) < OffsetDateTime::now_utc() ||
                        matches!(message.message_state, MessageState::MessageReceived) &&
                        message.last_updated + time::Duration::seconds(message_retry_timer) < OffsetDateTime::now_utc()
                        {
                            send_publish_message(message.packet_identifier, &mut client.tcp_stream, subscription.topic_title.to_string(), message.message.to_string(), false, subscription.sub_qos);
                            message.add_retry();
        
                            if message.retry_count >= max_retry_count {
                                message.update_state(MessageState::MessageUnsuccessful);
                            }
                        }
        
                        // Removes message from subscription message list if completed
                        if matches!(message.message_state, MessageState::MessageCompleted) ||
                        matches!(message.message_state, MessageState::MessageAcknowledged) ||
                        matches!(message.message_state, MessageState::None) ||
                        matches!(message.message_state, MessageState::MessageUnsuccessful)
                        {
                            completed_messages.push(message.packet_identifier);
                        }
                    }
                }
        
                // Prints all messages in subscription message list
                // TODO: Remove this when done testing
                //println!("Messages in subscription list:");
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
        
}

fn send_publish_message(packet_identifier: u16, stream: &mut TcpStream, topic_name: String, message: String, is_retry: bool, sub_qos: u8){
    let mut response: Vec<u8> = Vec::new();

    let mut message_type = MessageType::Publish.to_u8();

    if is_retry {
        message_type |= 0b00001000; // Sets the DUP flag to 1 - if message is a retry
    }

    message_type |= (sub_qos << 1) & 0b00000110; // Sets the QoS level

    response.push(message_type);
    response.push(0); // Remaining length (placeholder)
    response.push((topic_name.len() / 256) as u8); // Topic length MSB
    response.push((topic_name.len() % 256) as u8); // Topic length LSB
    
    response.extend_from_slice(topic_name.as_bytes());

    // Appends packet identifier if QoS level is 1 or 2
    if sub_qos > 0 {
        response.push((packet_identifier / 256) as u8); // Packet identifier MSB
        response.push((packet_identifier % 256) as u8); // Packet identifier LSB
    }
    
    // Appends the message (payload) to the response
    response.extend_from_slice(message.as_bytes());

    response[1] = response.len() as u8 - 2; // Subtract the fixed header length (2 bytes)

    println!("{:?}", response);

    send_response(stream, &response);
}