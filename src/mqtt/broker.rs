use std::{borrow::Borrow, net::TcpListener, sync::{Arc, Mutex}};
use rand::Rng;
use crate::mqtt::broker_state::Client;

use crate::mqtt::client_handler::handle_client;
use std::thread;

use super::broker_state::{self, BrokerState};


pub fn start_broker() -> () {
    let listener = TcpListener::bind("raspberrypi.local:6942");
    let mut total_clients: u8 = 0;
    let broker_state: Arc<Mutex<BrokerState>> = Arc::new(Mutex::new(broker_state::BrokerState::new()));
    for stream in listener.unwrap().incoming() {
        total_clients += 1;
        let broker_state: Arc<Mutex<BrokerState>> = Arc::clone(&broker_state);
        thread::spawn(move || {
            let thread_id = create_thread_id();
            handle_client(stream.unwrap(), broker_state, thread_id);
            
        });
    }
}

fn create_thread_id() -> f64{

    //Generate random thread id.
    let mut rng = rand::thread_rng();
    let thread_id: f64 = rng.gen(); 

    return thread_id;
}