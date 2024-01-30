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