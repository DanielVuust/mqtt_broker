use std::{sync::MutexGuard, time::SystemTime};

use time::{OffsetDateTime, PrimitiveDateTime};

use crate::mqtt::{broker_state::{BrokerState, Client}, utils::get_length};



pub fn handle_connect(buffer: &[u8], thread_id: f64, mut broker_state: MutexGuard<'_, BrokerState>){


    //Chceks protocol name;
    let protocol_name_len = get_length(&buffer, 3);
    let protocol_name = &buffer[4..4 + protocol_name_len];
    if protocol_name != b"MQTT" {
        return panic!("Invalid protocol name");
    }



    let mut client_id: String = "".to_string();    
    let mut will_topic: String  = "".to_string();
    let mut will_text: String  = "".to_string();
    //Chmut ange to two;bits 
    let mut will_qos: u8;
    let mut clean_session: bool;

    let mut current_index_in_buffer :usize; // Removed value 2
    //Skip to connectflag. 
    //TODO make sure its the right position.
    current_index_in_buffer = 9;

    let mut flag_byte = buffer[current_index_in_buffer];
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
    if flag_byte >= 1 {
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
    let keep_alive_seconds = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
    println!("keep_alive_secounds {:?}", keep_alive_seconds);
    //Clientid length MSB; TODO change to somehting else.
    current_index_in_buffer += 2;


    let client_id_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;

    for index in current_index_in_buffer+2..current_index_in_buffer+client_id_length+2 {
        client_id.push( buffer[index] as char);
    }
    println!("{}", client_id);

    current_index_in_buffer += client_id_length+2;
    
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

        // current_index_in_buffer += password_length+2;
    } 


    // //TODO check for correct position.
    // println!("{:?}", &buffer[2..4]);
    // // get_utf_8_string(&buffer[2..]);

    // let length: usize = buffer[2] as usize * 256 as usize + buffer[1] as usize;

    // for index in 2..length+2 {
    //     println!("{}", buffer[index]);
    // }

    let now = OffsetDateTime::now_utc();
    let mut hest = Client{
        thread_id,
        cancellation_requested: false,
        subscriptions: Vec::new(),
        last_connection: PrimitiveDateTime::new(now.date(), now.time()),
        client_id,
        will_topic,
        will_text,
        will_retain: will_retain_flag,
        will_qos: will_qos_flag,
        clean_session: clean_session_flag,
        keep_alive_seconds: keep_alive_seconds,
    };

    
        
    // (client_id, will_topic, will_text, will_retain_flag,
    //     will_qos_flag, clean_session_flag, keep_alive_secounds)
}