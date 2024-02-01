use std::time::Duration;
use std::thread;

mod common;
use crate::common::test_utils;
use crate::common::test_client::TestClient;

// Use this command: cargo test --test integration_test_connection

#[test]
fn subscribe_returns_suback_on_valid_connection() {
    // Starting broker
    let mut client = test_utils::start_test_broker();

    client = test_utils::create_valid_connection(client);

    let subscribe_command = vec![
        0x80, // MQTT Control Packet type for CONNECT
        0x16,   // Remaining Length (22 bytes)
        0x04, // Protocol Level (0x04 for MQTT 3.1.1)
        0x04, // Topic length
        0x54, 0x65, 0x73, 0x74, //Topic name
        0x00, //Topic QoS

    ];

    client.send_message(&subscribe_command).expect("Failed to read from the broker");
    
    let result = client.read_response().expect("Could not read");

    let expected_result = vec![
        
    ];

    assert_eq!(result, expected_result);

    // Closes the connection
    client.close().expect("Failed to close the connection");
}

