use std::thread;
use std::sync::Once;
use std::time::Duration;
use mqtt_broker::mqtt::broker;

use super::test_client::TestClient;

//let add_one = || 1 + 1;
//let result = add_one();
//let add_numbers = |x: i32, y: i32| x + y;
//let sum = add_numbers(5, 7);

// Thread safety - Ensures the broker is only started once, regardless of how many tests (threads) are run
static INIT: Once = Once::new();

pub fn start_test_broker() -> TestClient{
    INIT.call_once(|| {
        thread::spawn(|| {
            broker::start_broker();
        });
    });
    thread::sleep(Duration::from_secs(1));

    TestClient::connect("127.0.0.1:6942").expect("Failed to connect to the broker")

}

pub fn create_valid_connection(mut client: TestClient) -> TestClient{
    // Creates a Connect message
    let connect_msg = construct_connect_message();

    // Sends the Connect message to the broker
    client.send_message(&connect_msg).expect("Failed to send Connect message");

    // Reads the response from the broker (expects: Connack message)
    let response = client.read_response().expect("Failed to read from the broker");

    // Validates the response
    assert_eq!(response, construct_connack_message());

    client
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