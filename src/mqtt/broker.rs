use std::net::TcpListener;
use crate::mqtt::client_handler::handle_client;
use std::thread;

pub fn start_broker() -> () {
    let listener = TcpListener::bind("127.0.0.1:6942");
    for stream in listener.unwrap().incoming() {
        
        thread::spawn(move || {
            match stream {
                Ok(stream) => {
                    if let Err(e) = handle_client(stream) {
                        println!("An error occurred while handling client: {}", e);
                    }
                },
                Err(e) => println!("Connection failed: {}", e),
            }
        });
    }
}