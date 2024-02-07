use std::net::Ipv4Addr;

pub fn log_error(message: &str) {
    println!("Error: {}", message);
}

pub fn log_event(message: &str) {
    println!("Event: {}", message);
}

pub fn validate_ip_address(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from("Invalid UTF-8"))
}

// Get the length of message
pub fn get_message_length(buffer: &[u8], position: usize) -> usize {
    
    // If the least significant bit is smaller than 128, the length is 1 byte long
    if buffer[position + 1] < 128 {
        return buffer[position] as usize;
    }

    // If the least significant bit is larger than 128, the length is 2 bytes long
    let len = ((buffer[position] & 0x7F) as usize) << 8 | buffer[(position + 1) as usize] as usize;

    len
}