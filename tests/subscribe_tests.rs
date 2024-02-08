
mod common;
use crate::common::mqtt_commands_utils::{connack_vec, connect_vec, suback_vec, subscribe_vec};
use crate::common::test_utils::{connect_to_broker, start_test_broker};
use crate::common::test_client::TestClient;

// Use this command: cargo test --test subscribe_tests

#[test]
fn subscribe_successfully_to_topic_test() {

    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Connects to the broker using a connect request
    client = connect_to_broker(client, "test".to_string(), 60);
    
    //Package identifier to be sent to the broker
    let package_identifier: u16 = 32482;

    //Creates the subscribe request to be sent to the broker
    let subscribe_request = subscribe_vec(vec!["test".to_string()], vec![0], package_identifier);

    //Sends the subscribe reqest to the broker.
    client.send_message(&subscribe_request).unwrap();

    //Get the response from the broker.
    let response = client.read_response().unwrap();

    //Checks if the suback respnse is the expected result 
    assert_eq!(response, suback_vec(vec![0], package_identifier));

    // Closes the connection
    client.close().unwrap();
}

#[test]
fn subscribe_successfully_to_topic_with_qos_1_test() {

    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Connects to the broker using a connect request
    client = connect_to_broker(client, "test".to_string(), 60);
    
    //Package identifier to be sent to the broker
    let package_identifier: u16 = 32482;

    //Creates the subscribe request to be sent to the broker
    let subscribe_request = subscribe_vec(vec!["test".to_string()], vec![1], package_identifier);

    //Sends the subscribe reqest to the broker.
    client.send_message(&subscribe_request).unwrap();

    //Get the response from the broker.
    let response = client.read_response().unwrap();

    //Checks if the suback respnse is the expected result 
    assert_eq!(response, suback_vec(vec![1], package_identifier));

    // Closes the connection
    client.close().unwrap();
}

#[test]
fn subscribe_successfully_to_topic_with_qos_2_test() {

    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Connects to the broker using a connect request
    client = connect_to_broker(client, "test".to_string(), 60);

    //Package identifier to be sent to the broker
    let package_identifier: u16 = 32482;

    //Creates the subscribe request to be sent to the broker
    let subscribe_request = subscribe_vec(vec!["test".to_string()], vec![2], package_identifier);
    
    //Sends the subscribe reqest to the broker.
    client.send_message(&subscribe_request).unwrap();
    
    //Get the response from the broker.
    let response = client.read_response().unwrap();

    //Checks if the suback respnse is the expected result 
    assert_eq!(response, suback_vec(vec![2], package_identifier));

    // Closes the connection
    client.close().unwrap();
}


#[test]
fn subscribe_successfully_to_topics_with_differnet_qos_test() {

    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Connects to the broker using a connect request
    client = connect_to_broker(client, "test".to_string(), 60);

    //Package identifier to be sent to the broker
    let package_identifier: u16 = 32482;

    //Creates the subscribe request to be sent to the broker
    let subscribe_request = subscribe_vec(vec!["test".to_string(), "test2".to_string(), "test3".to_string()], vec![2, 0, 1], package_identifier);

    //Sends the subscribe reqest to the broker.
    client.send_message(&subscribe_request).unwrap();

    //Get the response from the broker.
    let response = client.read_response().unwrap();

    //Checks if the suback respnse is the expected result 
    assert_eq!(response, suback_vec(vec![2, 0, 1], package_identifier));

    // Closes the connection
    client.close().unwrap();
}
