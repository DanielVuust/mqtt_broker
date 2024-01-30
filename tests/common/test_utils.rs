use std::thread;
use std::sync::Once;
use mqtt_broker::mqtt::broker;

static INIT: Once = Once::new();

pub fn start_test_broker() {
    INIT.call_once(|| {
        thread::spawn(|| {
            broker::start_broker().expect("Broker failed to start");
        });
    });
}