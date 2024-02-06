use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::{Duration, SystemTime};
use crate::mqtt::message_handlers::connect_handler::handle_connect;
use crate::mqtt::message_handlers::ping_handler::ping_resp;
use crate::mqtt::message_handlers::publish_handler::handle_publish;
use crate::mqtt::message_handlers::subscribe_handler::handle_subscribe;
use crate::mqtt::message_sender::{ send_response};
use crate::mqtt::message_type::MessageType;
use time::{OffsetDateTime, PrimitiveDateTime};

use super::broker_state::{BrokerState, SubscriptionMessage};


// Handles client connection
pub async fn handle_client(mut stream: TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64) {
    println!("{}", thread_id);
    let mut buffer = [0; 2048];
    let mut client_id: String = "".to_string();    
    let mut will_topic: String  = "".to_string();
    let mut will_text: String  = "".to_string();
    let mut will_retain: bool;
    let mut will_qos: u8;
    let mut clean_session: bool;
    let mut keep_alive_secounds: usize;
    let mut last_communication: SystemTime;
    let mut first_stream = stream.try_clone().expect("Cannot clone stream");
    let mut second_stream = stream.try_clone().expect("Cannot clone stream");
    
    let callback_closure = |result: i32| {

    };  
    let runtime = tokio::runtime::Runtime::new().unwrap(); 
    let arc2 = Arc::clone(&arc_broker_state);
    thread::spawn(move || {
        let _ = runtime.block_on(runtime.spawn(async move {
            handle_second_stream(&mut second_stream, arc2, thread_id, callback_closure);
        }));
    });
    
    println!("continue");

    // Reads data from stream until connection is closed
    while match first_stream.read(&mut buffer) {
        
        Ok(size) => {
            let mut current_broker_state = arc_broker_state.lock().unwrap();
            let currect_client = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id ).unwrap().1;

            if size == 0 {
                // No data received, closing connection
                false
            } else {
                let now = OffsetDateTime::now_utc();
                currect_client.last_connection = PrimitiveDateTime::new(now.date(), now.time());
                match MessageType::from_u8(buffer[0]) {
                    // Connect
                    Some(MessageType::Connect) => {
                        println!("CONNECT message received");
                        (client_id, will_topic, will_text, will_retain, will_qos, clean_session, keep_alive_secounds) = handle_connect(&buffer);
                        
                        currect_client.client_id = client_id;

                        let mut response: [u8; 4] = [0; 4];
                        response[0] = MessageType::Connack.to_u8();
                        response[1] = 2;
                        send_response(&mut stream, &response);
                        
                        
                    }
                    // Publish
                    Some(MessageType::Publish) =>{
                        println!("PUBLISH message received");
                        handle_publish(&mut stream, &buffer, thread_id, current_broker_state);
                       return;

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

fn handle_second_stream( stream: &mut TcpStream, arc_broker_state: Arc<Mutex<BrokerState>>, thread_id: f64, callback: fn(i32)){
    
    loop{
        sleep(Duration::from_millis(750));
        let mut current_broker_state = arc_broker_state.lock().unwrap();
        let client = (*current_broker_state).clients.iter_mut().enumerate().find(| x: &(usize, &mut crate::mqtt::broker_state::Client) | &x.1.thread_id == &thread_id ).unwrap().1;
        
        for j in client.subscriptions.iter_mut().enumerate() {
            for (index, message) 
                in j.1.messages.iter_mut().enumerate()
                    .find(|x| x.1.message_sent == false)  { 
                send_publish_message(stream, j.1.topic_title.to_string(), message.message.to_string());
                message.message_sent = true;
            }
        }
        let now = OffsetDateTime::now_utc();
        println!("{:?}", client.last_connection);
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
    // let mut response: Vec<u8> = [48, 26, 0, 4, 102, 117, 99, 107, 123, 10, 32, 32, 34, 109, 115, 103, 34, 58, 32, 34, 104, 101, 108, 108, 111, 34, 10, 125].to_vec();
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
    response.push(0);
    response.push(0);
    response.push(0);
    response.push(0);
    response.push(0);
    
    response[1] = response.len() as u8 - 2;

    println!("{:?}", response);

    send_response(stream, &response);
}
