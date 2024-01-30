use std::thread;
use std::time::Duration;

pub fn start_test_broker() {
    thread::spawn(|| {
        mqtt_broker::start_broker().unwrap();
    });

    // Wait for the broker to start
    // thread::sleep(Duration::from_secs(1));
}