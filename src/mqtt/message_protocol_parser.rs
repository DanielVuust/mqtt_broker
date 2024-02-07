use crate::mqtt::utils::get_message_length;

// Handles CONNECTION parsing
pub fn parse_connect_message(buffer: &[u8]) -> Result<(), String> {
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
    //let protocol_name_len = ((buffer[2] as usize) << 8) | buffer[3] as usize;

    let protocol_name_len = get_message_length(buffer, 3);

    // Get's the protocol name from the buffer[start..end]
    let protocol_name = &buffer[4..4 + protocol_name_len];

    // Checks if protocol name is valid
    if protocol_name != b"MQTT" {
        return Err("Invalid protocol name".to_string());
    }

    
    Ok(())
}