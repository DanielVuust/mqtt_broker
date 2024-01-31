mod message_type;
use message_type::MessageType;
use core::panic;
use std::{net::{TcpListener, TcpStream}, string};
mod message_reader;


#[allow(unused_imports)]
use std::io::{Read, Write};

use crate::message_reader::{create_client, get_utf_8_string};

// Handles client connection
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

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

                // Handles client message types
                println!("{:?}", buffer);

                match MessageType::from_u8(msg_type) {
                    // Connect
                    Some(MessageType::Connect) => {
                        match parse_connect_message(&buffer[..size]) {
                            Ok(_) => {
                                println!("CONNECT message received");
                                let mut current_index_in_buffer :usize = 2;
                                //Skip to connectflag. 
                                //TODO make sure its the right position.
                                current_index_in_buffer = 9;

                                let mut flag_byte = buffer[current_index_in_buffer];
                                println!("{:?}", flag_byte);
                                let mut username_flag: bool = false;
                                let mut password_flag: bool = false;
                                let mut will_retain_flag: bool = false;
                                let mut will_qos_flag: u8 = 0;
                                let mut will_flag: bool = false;
                                let mut clean_session_flag: bool = false;
                                if flag_byte >= 128 {
                                    username_flag = true;
                                    flag_byte -= 128;
                                }
                                if flag_byte >= 64 {
                                    password_flag = true;
                                    flag_byte -= 64;
                                }
                                if flag_byte >= 32 {
                                    will_retain_flag = true;
                                    flag_byte -= 32
                                }
                                if flag_byte >= 16 {
                                    will_qos_flag += 2;
                                    flag_byte -= 16;
                                }
                                if flag_byte >= 8 {
                                    will_qos_flag += 1;
                                    flag_byte -= 8;
                                }
                                if flag_byte >= 4 {
                                    will_flag = true;
                                    flag_byte -= 4;
                                }
                                if flag_byte >= 2 {
                                    clean_session_flag = true;
                                    flag_byte -=2;
                                }
                                if(flag_byte >= 1){
                                    //TODO return correct status code to client :\
                                    panic!("LSb in flag byte is reserved")
                                }
                                println!("username_flag {:?}", username_flag);
                                println!("password_flag {:?}", password_flag);
                                println!("will_qos_flag {:?}", will_qos_flag);
                                println!("will_flag {:?}", will_flag);
                                println!("clean_session_flag {:?}", clean_session_flag);

                                //Keep alive MSB; TODO change to something else.
                                current_index_in_buffer += 1;
                                let keep_alive_secounds = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
                                println!("keep_alive_secounds {:?}", keep_alive_secounds);
                                //Clientid length MSB; TODO change to somehting else.
                                current_index_in_buffer += 2;


                                let client_id_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
                                let mut client_id: String = "".to_string();

                                for index in current_index_in_buffer+2..current_index_in_buffer+client_id_length+2 {
                                    client_id.push( buffer[index] as char);
                                }
                                println!("{}", client_id);
                                println!("{}", current_index_in_buffer);

                                current_index_in_buffer += client_id_length+2;
                                
                                let mut will_topic: String = "".to_string();
                                let mut will_text: String = "".to_string();
                                if will_flag {
                                    let will_topic_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
    
                                    for index in current_index_in_buffer+2..current_index_in_buffer+will_topic_length+2 {
                                        will_topic.push( buffer[index] as char);
                                    }
                                    println!("{}", will_topic);
    
                                    current_index_in_buffer += will_topic_length+2;



                                    let will_text_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
    
                                    for index in current_index_in_buffer+2..current_index_in_buffer+will_text_length+2 {
                                        will_text.push( buffer[index] as char);
                                    }
                                    println!("{}", will_text);
    
                                    current_index_in_buffer += will_text_length+2;
                                }

                                let mut username: String = "".to_string();
                                if username_flag {
                                println!("current_index_in_buffer {:?}", current_index_in_buffer);

                                    let username_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
                                    
                                    println!("current_index_in_buffer + 2 {:?}", current_index_in_buffer+2);
                                    println!("username_length+2 {:?}", username_length+2);

                                    for index in current_index_in_buffer+2..current_index_in_buffer+username_length+2 {
                                        username.push( buffer[index] as char);
                                        println!("username {:?}", username);
                                    }
                                    println!("{}", username);
    
                                    current_index_in_buffer += username_length+2;
                                } 
                                let mut password: String = "".to_string();
                                if password_flag {
                                    let password_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
    
                                    for index in current_index_in_buffer+2..current_index_in_buffer+password_length+2 {
                                        password.push( buffer[index] as char);
                                    }
                                    println!("{}", password);
    
                                    current_index_in_buffer += password_length+2;
                                } 


                                // //TODO check for correct position.
                                // println!("{:?}", &buffer[2..4]);
                                // // get_utf_8_string(&buffer[2..]);

                                // let length: usize = buffer[2] as usize * 256 as usize + buffer[1] as usize;

                                // for index in 2..length+2 {
                                //     println!("{}", buffer[index]);
                                // }

                                create_client(vec![], client_id, username, password, will_flag, will_text, will_topic, will_retain_flag, will_qos_flag, clean_session_flag, keep_alive_secounds);
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
                    }
                    // Unsubscribe
                    Some(MessageType::Unsubscribe) =>{
                        println!("Unsubscribe message received");
                    }
                    // Pingreq
                    Some(MessageType::Pingreq) =>{
                        println!("PINGREQ message received");
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

// Handles CONNECTION parsing
fn parse_connect_message(buffer: &[u8]) -> Result<(), String> {
    // Checks minimum length for CONNECT message
    if buffer.len() < 14 {
        return Err("Message too short to be a valid CONNECT message".to_string());
    }

    // Parsing of protocol name length
    // The protocol name length is stored in the third and fourth bytes of the buffer (buffer[2] and buffer[3]).
    // These two bytes represent a 16-bit integer in big-endian format.

    // The following takes the first 8 bits of the buffer (buffer[2]) and shifts them to the left by 8 bits
    // Example: '0000 0000 0000 0000 0000 0000 0000 0100' << 8 = '0000 0000 0000 0100 0000 0000 0000 0000'
    // Then takes the next 8 bits of the buffer and adds them to the previous result
    // Example: '0000 0000 0000 0100 0000 0000 0000 0000' | '0000 0000 0000 0000 0000 0000 0000 0100' = '0000 0000 0000 0100 0000 0000 0000 0100'
    // This gets us the length of the protocol name and converts it to usize
    let protocol_name_len = ((buffer[2] as usize) << 8) | buffer[3] as usize;

    // Get's the protocol name from the buffer[start..end]
    let protocol_name = &buffer[4..4 + protocol_name_len];

    // Checks if protocol name is valid
    if protocol_name != b"MQTT" {
        return Err("Invalid protocol name".to_string());
    }

    
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6942")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Connection failed: {}", e),
        }
    }
    Ok(())
}