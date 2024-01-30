mod common;
use common::test_client::TestClient;

use std::thread;
use std::time::Duration;

fn

mod tests {
    use super::*;
    use std::thread;

    // Helper function to start the broker for testing
    fn start_test_broker() {
        thread::spawn(|| {
            main().unwrap(); // Replace this with the actual function to start your broker
        });
    }

    #[test]
    fn test_mqtt_connect() {
        start_test_broker(); // Start the broker in a separate thread

        // Give some time for the broker to start up
        thread::sleep(Duration::from_secs(1));

        // Connect to the broker
        let mut client = TestClient::connect("127.0.0.1:6942").unwrap();

        // Send a valid Connect message
        let connect_msg = [/* bytes representing a valid Connect message */];
        client.send_message(&connect_msg).unwrap();

        // Read the response from the broker
        let response = client.read_response().unwrap();

        // Assert that the response is a Connack message
        assert_eq!(response, [/* expected Connack message bytes */]);

        client.close().unwrap();
    }
}