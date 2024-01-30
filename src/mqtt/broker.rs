use std::net::TcpListener;
use crate::mqtt::client_handler::handle_client;

pub fn start_broker() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6942")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),  // Your existing client handling logic
            Err(e) => println!("Connection failed: {}", e),
        }
    }
    Ok(())
}