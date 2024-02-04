use std::{borrow::Borrow, net::TcpListener, sync::{Arc, Mutex}};
use rand::Rng;
use crate::mqtt::broker_state::Client;

use crate::mqtt::client_handler::handle_client;
use std::thread;

use super::broker_state::{self, BrokerState};


pub fn start_broker() -> () {
    let listener = TcpListener::bind("127.0.0.1:6942");
    let mut total_clients: u8 = 0;
    let broker_state: Arc<Mutex<BrokerState>> = Arc::new(Mutex::new(broker_state::BrokerState::new()));
    for stream in listener.unwrap().incoming() {
        total_clients += 1;
        let broker_state: Arc<Mutex<BrokerState>> = Arc::clone(&broker_state);
        let runtime = tokio::runtime::Runtime::new().unwrap(); 
        thread::spawn(move || {
            let thread_id: f64;
            (thread_id) = create_new_client(&broker_state);
            let _ = runtime.block_on(runtime.spawn(async move {
                handle_client(stream.unwrap(), broker_state, thread_id).await;
            }));    
        });
    }
}

fn create_new_client(broker_state: &Arc<Mutex<BrokerState>>) -> f64{

    //Generate random thread id.
    let mut rng = rand::thread_rng();
    let thread_id: f64 = rng.gen(); 
    
    let mut t = broker_state.lock().unwrap();
    let new_client: Client = Client::new(thread_id);
    
    (*t).clients.push(new_client);

    return thread_id;
}