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

// Get length
pub fn get_length(buffer: &[u8], position: u8) -> usize {

    // If the first bit is 0, the length is 1 byte long
    if buffer[position as usize] < 128 {
        return buffer[position as usize] as usize;
    }

    // If the first bit is 1, the length is 2 bytes long (& 0x7F removes the first bit)
    let len = ((buffer[position as usize] & 0x7F) as usize) << 8 | buffer[(position + 1) as usize] as usize;

    len
}