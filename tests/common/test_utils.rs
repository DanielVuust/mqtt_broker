use std::thread;
use std::sync::Once;
use mqtt_broker::mqtt::broker;

//let add_one = || 1 + 1;
//let result = add_one();
//let add_numbers = |x: i32, y: i32| x + y;
//let sum = add_numbers(5, 7);

// Thread safety - Ensures the broker is only started once, regardless of how many tests (threads) are run
static INIT: Once = Once::new();

pub fn start_test_broker() {
    INIT.call_once(|| {
        thread::spawn(|| {
            broker::start_broker().expect("Broker failed to start");
        });
    });
}