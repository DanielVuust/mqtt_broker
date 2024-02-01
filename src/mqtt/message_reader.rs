
use std::{net::TcpStream, time::SystemTime};


pub fn get_utf_8_string(buffer: &[u8]) -> String {
    
    let length: usize = buffer[0] as usize * 256 as usize + buffer[1] as usize;
    
    for index in 2..length+2 {
        println!("{}", buffer[index]);
    }
    
    "4res2".to_string()
}
