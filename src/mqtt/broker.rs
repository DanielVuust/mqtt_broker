use std::{net::TcpListener, sync::{Arc, Mutex}};
use rand::Rng;

use crate::mqtt::client_handler::handle_client;
use std::thread;

use super::{broker_state::{self, BrokerState}, state_thread::state_keeper_loop};


pub fn start_broker() -> () {

    //Creates a lisiner to listen for new connections to the broker.
    let listener = TcpListener::bind("0.0.0.0:7878");

    //Creates global state that is shared between threads.
    let broker_state: Arc<Mutex<BrokerState>> = Arc::new(Mutex::new(broker_state::BrokerState::new()));

    //Clones the broker state and starts the thread to keep track of state actions.
    let cloned_broker_state = Arc::clone(&broker_state);
    start_thread_to_do_state_actions(cloned_broker_state);


    //Listen for new tcp connections and create threads when one connects.
    for stream in listener.unwrap().incoming() {
        let cloned_broker_state = Arc::clone(&broker_state);
        thread::spawn(move || {
            let thread_id = create_random_thread_id();
            handle_client(stream.unwrap(), cloned_broker_state, thread_id);
            
        });
    }
}

fn create_random_thread_id() -> f64{

    //Generate random thread id.
    let mut rng = rand::thread_rng();
    let thread_id: f64 = rng.gen(); 

    return thread_id;
}

fn start_thread_to_do_state_actions(broker_state:  Arc<Mutex<BrokerState>>){
    thread::spawn(move || {
        state_keeper_loop(broker_state)
    });
}
