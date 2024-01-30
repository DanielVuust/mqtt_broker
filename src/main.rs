use mqtt_broker::mqtt;

fn main() -> std::io::Result<()> {
    mqtt::broker::start_broker()
}