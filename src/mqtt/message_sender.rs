use std::io::Write;
use std::net::TcpStream;

// Returns an answer to the client
pub fn send_response(stream: &mut TcpStream, buffer: &[u8], ) {
    println!("Sending message HERERHERHEHRHEHRHEHREHRHEHRHEHRHEHRHEHRHEHREHR: {:?}", buffer);

    // Sends message and checks errors
    match stream.write(buffer) {
        Ok(_) => {
            println!("Message sent");
        }
        Err(e) => {
            println!("Failed to send message: {}", e);
        }
    }

    // Flushes the stream
    match stream.flush() {
        Ok(_) => {
            println!("Stream flushed");
        }
        Err(e) => {
            println!("Failed to flush stream: {}", e);
        }
    }
}
