use mqtt_broker::mqtt;

fn main() -> () {
    mqtt::broker::start_broker();
}