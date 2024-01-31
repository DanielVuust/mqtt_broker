
use std::{io::Read, time::{SystemTime}};



pub fn create_client(mut clients: Vec<Client>,
    client_id: String, username: String, password: String, will_flag: bool, will_text: String, will_topic: String, will_retain_flag: bool, will_qos_flag: u8, clean_session_flag: bool, keep_alive_secounds: usize ) -> bool{
    let mut cilent: Client = Client{
        client_id: client_id,
        username: username,
        password: password,
        will: will_flag,
        will_text: will_text,
        will_topic: will_topic,
        will_retain: will_retain_flag,
        will_qos: will_qos_flag,
        clean_session: clean_session_flag,
        keep_alive_secounds: keep_alive_secounds,
        last_communication: SystemTime::now()
    };
    clients.push(cilent);
    println!("{:?}", clients);
    true
}

pub fn get_utf_8_string(buffer: &[u8]) -> String {

    let length: usize = buffer[0] as usize * 256 as usize + buffer[1] as usize;

    for index in 2..length+2 {
        println!("{}", buffer[index]);
    }

    "4res2".to_string()
}

#[derive(Debug)]
pub struct Client {
    client_id: String,
    
    //Do we need this??
    username: String,
    password: String,
    
    will: bool ,
    will_topic: String,
    will_text: String,
    will_retain: bool,
    //Change to two bits 
    will_qos: u8,
    clean_session: bool,
    keep_alive_secounds: usize,
    last_communication: SystemTime
}
// impl Client {
//     fn new() -> Self{
//         Client{
//             client_id: "".to_string(),
//             username: "".to_string(),
//             password: "".to_string(),
//             will: false,
//             will_topic: "".to_string(),
//             will_retain: false,
//             will_qos: 1,
//             clean_session: false,
//             keep_alive_secounds: 60,
//             last_communication: SystemTime::now(),
//         }
//     }
// }