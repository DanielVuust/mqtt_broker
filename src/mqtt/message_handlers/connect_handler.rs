use std::time::SystemTime;
use std::io::Result;

pub fn handle_connect(buffer: &[u8]) -> (String, String, String, bool,
    u8, bool, usize){

    let mut client_id = String::new();
    let mut will_topic = String::new();
    let mut will_text = String::new();
    
    let mut current_index_in_buffer :usize;

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
    let keep_alive_secounds = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;
    println!("keep_alive_secounds {:?}", keep_alive_secounds);
    //Clientid length MSB; TODO change to somehting else.
    current_index_in_buffer += 2;


    let client_id_length: usize = buffer[current_index_in_buffer] as usize * 256 as usize + buffer[current_index_in_buffer + 1] as usize;

    for index in current_index_in_buffer+2..current_index_in_buffer+client_id_length+2 {
        client_id.push( buffer[index] as char);
    }
    println!("{}", client_id);
    println!("{}", current_index_in_buffer);

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
    (client_id, will_topic, will_text, will_retain_flag,
        will_qos_flag, clean_session_flag, keep_alive_secounds)
}