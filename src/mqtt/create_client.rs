use std::{net::TcpStream, sync::MutexGuard};

use crate::mqtt::broker_state::Client;

use super::broker_state::BrokerState;

pub fn create_client (buffer: &[u8], thread_id: f64, mut broker_state: MutexGuard<'_, BrokerState>, stream:  &mut TcpStream){
    let mut current_index_in_buffer :usize; // Removed value 2
    //Skip to connectflag. 
    //TODO make sure its the right position.
    current_index_in_buffer = 9;

    let flag_byte = buffer[current_index_in_buffer];
    let (username_flag, password_flag, will_retain_flag, will_qos_flag, will_flag, clean_session_flag) 
        = decode_flags(flag_byte);

    println!("username_flag: {:?}", username_flag);
    println!("password_flag: {:?}", password_flag);
    println!("will_retain_flag: {:?}", will_retain_flag);
    println!("will_qos_flag: {:?}", will_qos_flag);
    println!("will_flag: {:?}", will_flag);
    println!("clean_session_flag: {:?}", clean_session_flag);

    //Keep alive MSB; TODO change to something else.
    current_index_in_buffer += 1;
    
    let (keep_alive_seconds, new_index) = extract_keep_alive_seconds(&buffer, current_index_in_buffer);
    current_index_in_buffer = new_index;

    let client_id_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;

    let (client_id, new_index) = extract_client_id(&buffer, current_index_in_buffer, client_id_length);
    current_index_in_buffer = new_index;

    let (will_topic, will_text, new_index) = extract_will_message(&buffer, current_index_in_buffer, will_flag);
    
    current_index_in_buffer = new_index;
    // let username = extract_username(buffer, username_flag, current_index_in_buffer);
    // let password = extract_password(buffer, password_flag, current_index_in_buffer);

        let new_client: Client = Client::new(
        thread_id, 
        client_id,
        will_topic,
        will_text,
        will_retain_flag,
        will_qos_flag,
        clean_session_flag,
        keep_alive_seconds,
    Vec::new(), false,
        stream.try_clone().unwrap());
    (*broker_state).clients.push(new_client);
}

fn decode_flags(mut flag_byte: u8) -> (bool, bool, bool, u8, bool, bool) {
    let mut username_flag = false;
    let mut password_flag = false;
    let mut will_retain_flag = false;
    let mut will_qos_flag = 0;
    let mut will_flag = false;
    let mut clean_session_flag = false;

    // Decode flags from the flag_byte
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
        flag_byte -= 32;
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
        flag_byte -= 2;
    }
    if flag_byte >= 1 {
        panic!("LSb in flag byte is reserved");
    }

    // Return the decoded flags
    (username_flag, password_flag, will_retain_flag, will_qos_flag, will_flag, clean_session_flag)
}

fn extract_keep_alive_seconds(buffer: &[u8], current_index_in_buffer: usize) -> (usize, usize) {
    // Extract keep-alive seconds
    let keep_alive_seconds = buffer[current_index_in_buffer] as usize * 256 + buffer[current_index_in_buffer + 1] as usize;
    ;
    // Print the extracted keep-alive seconds
    println!("keep_alive_seconds: {:?}", keep_alive_seconds);

    // Return the extracted keep-alive seconds and the new index in the buffer
    (keep_alive_seconds, current_index_in_buffer + 2)
}

fn extract_client_id(buffer: &[u8], current_index_in_buffer: usize, client_id_length: usize) -> (String, usize) {
    let mut client_id = String::new();

    // Extract client ID
    for index in current_index_in_buffer + 2..current_index_in_buffer + client_id_length + 2 {
        client_id.push(buffer[index] as char);
    }

    // Print the extracted client ID
    println!("{}", client_id);

    // Return the extracted client ID and the new index in the buffer
    (client_id, current_index_in_buffer + client_id_length + 2)
}


fn extract_will_message(buffer: &[u8], current_index_in_buffer: usize, will_flag: bool) -> (String, String, usize) {
    let mut will_topic = String::new();
    let mut will_text = String::new();
    let mut new_index = current_index_in_buffer;

    if will_flag {
        let will_topic_length = buffer[new_index] as usize * 256 + buffer[new_index + 1] as usize;
        
        // Extract will topic
        for index in new_index + 2..new_index + will_topic_length + 2 {
            will_topic.push(buffer[index] as char);
        }
        println!("Will Topic: {}", will_topic);

        // Update index in buffer
        new_index += will_topic_length + 2;

        let will_text_length = buffer[new_index] as usize * 256 + buffer[new_index + 1] as usize;
        
        // Extract will text
        for index in new_index + 2..new_index + will_text_length + 2 {
            will_text.push(buffer[index] as char);
        }
        println!("Will Text: {}", will_text);

        // Update index in buffer
        new_index += will_text_length + 2;
    }

    (will_topic, will_text, new_index)
}

//To be used when we want our mqtt to have a username and password 
fn extract_password(buffer: &[u8], password_flag: bool, current_index_in_buffer: usize) -> String {
    let mut password = String::new();

    if password_flag {
        let password_length = buffer[current_index_in_buffer] as usize * 256 + buffer[current_index_in_buffer + 1] as usize;

        for index in current_index_in_buffer + 2..current_index_in_buffer + password_length + 2 {
            password.push(buffer[index] as char);
        }
        println!("{}", password);
    }

    password
}

fn extract_username(buffer: &[u8], username_flag: bool, mut current_index_in_buffer: usize) -> String {
    let mut username = String::new();

    if username_flag {
        println!("current_index_in_buffer {:?}", current_index_in_buffer);

        // Calculate the length of the username
        let username_length = buffer[current_index_in_buffer] as usize * 256 + buffer[current_index_in_buffer + 1] as usize;

        println!("current_index_in_buffer + 2 {:?}", current_index_in_buffer + 2);
        println!("username_length+2 {:?}", username_length + 2);

        // Extract characters of the username from the buffer
        for index in current_index_in_buffer + 2..current_index_in_buffer + username_length + 2 {
            username.push(buffer[index] as char);
            println!("username {:?}", username);
        }

        println!("{}", username);

        // Update the index in the buffer
        current_index_in_buffer += username_length + 2;
        let _ = current_index_in_buffer;
    }

    username
}


