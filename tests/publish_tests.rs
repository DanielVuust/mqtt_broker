
mod common;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use common::mqtt_commands_utils::publish_vec;
use common::test_utils::subscribe_to_topics;

use crate::common::test_utils::{connect_to_broker, start_test_broker};
use crate::common::test_client::TestClient;

// Use this command: cargo test --test publish_tests

#[test]
fn publish_to_topic_test() {

    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Connects to the broker using a connect request
    client = connect_to_broker(client, "test".to_string(), 60);
    
    //Package identifier to be sent to the broker
    let package_identifier: u16 = 32482;

    //Subscribes to the topic test
    client = subscribe_to_topics(client, package_identifier, vec!["test".to_string()], vec![0]);

    let publish_command = publish_vec("test".to_string(), 0, "testMessage".to_string(), 0 );

    client.send_message(&publish_command).unwrap();

    let mut buf = vec![0; 1024];

    client.stream.read(&mut buf).expect("msg");


    assert_eq!(buf, publish_command);
    // Closes the connection
    client.close().unwrap();
}
