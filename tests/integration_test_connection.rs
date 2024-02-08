use std::time::Duration;
use std::thread;

mod common;
use crate::common::test_utils::start_test_broker;
use crate::common::test_client::TestClient;

// Use this command: cargo test --test integration_test_connection

#[test]
fn connect_message_test() {
    // Starting broker
    start_test_broker();
    thread::sleep(Duration::from_secs(1));

    // Connecting TestClient to the broker
    let mut client = TestClient::connect("127.0.0.1:7878").expect("Failed to connect to the broker");

    // Creates a Connect message
    let connect_msg = construct_connect_message();

    // Sends the Connect message to the broker
    client.send_message(&connect_msg).expect("Failed to send Connect message");

    // Reads the response from the broker (expects: Connack message)
    let response = client.read_response().expect("Failed to read from the broker");

    // Validates the response
    assert_eq!(response, construct_connack_message());

    // Closes the connection
    client.close().expect("Failed to close the connection");
}

fn construct_connect_message() -> Vec<u8> {
    // Minimal CONNECT packet
    vec![
        0x10, // MQTT Control Packet type for CONNECT
        0x16,   // Remaining Length (22 bytes)
        0x00, 0x04, // Protocol Name Length
        0x4D, 0x51, 0x54, 0x54, // Protocol Name "MQTT"
        0x04, // Protocol Level (0x04 for MQTT 3.1.1)
        0x02, // Connect Flags (Clean Session)
        0x00, 0x3C, // Keep Alive (60 seconds)
        0x00, 0x0A, // Client Identifier Length (10 bytes for "TestClient")
        0x54, 0x65, 0x73, 0x74, 0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74, // Client Identifier "TestClient"
    ]
}

fn construct_connack_message() -> Vec<u8> {
    // Minimal CONNACK packet
    vec![0x20, 0x02, 0x00, 0x00]
}