use std::thread;
use std::sync::Once;
use mqtt_broker::mqtt::broker;

use crate::common::mqtt_commands_utils::{connack_vec, connect_vec, suback_vec, subscribe_vec};

use super::test_client::TestClient;

//let add_one = || 1 + 1;
//let result = add_one();
//let add_numbers = |x: i32, y: i32| x + y;
//let sum = add_numbers(5, 7);

// Thread safety - Ensures the broker is only started once, regardless of how many tests (threads) are run
static INIT: Once = Once::new();

pub fn start_test_broker() {
    INIT.call_once(|| {
        thread::spawn(|| {
            broker::start_broker();
        });
    });
}

pub fn connect_to_broker(mut client: TestClient, client_identifier: String, keep_alive: u16) -> TestClient{
    //Creates a connect request 
    let connect_request = connect_vec(0, keep_alive, client_identifier);

    //Sends the connect request to the borker
    client.send_message(&connect_request).unwrap();

    //Reads the response from the server
    let response = client.read_response().unwrap();

    // Validates if the response is a conack
    assert_eq!(response, connack_vec());
    
    client
}

pub fn subscribe_to_topics(mut client: TestClient, package_identifier: u16, topics: Vec<String>, topic_qoses: Vec<u8>) -> TestClient{
    
    //Creates the subscribe request to be sent to the broker
    let subscribe_request = subscribe_vec(topics.clone(), topic_qoses.clone(), package_identifier);

    //Sends the subscribe reqest to the broker.
    client.send_message(&subscribe_request).unwrap();

    //Get the response from the broker.
    let response = client.read_response().unwrap();

    //Checks if the suback respnse is the expected result 
    assert_eq!(response, suback_vec(topic_qoses, package_identifier));

    client
}