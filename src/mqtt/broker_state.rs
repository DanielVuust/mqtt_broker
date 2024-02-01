use std::string;

#[derive(Debug)]
pub struct BrokerState {
    pub clients: Vec<Client>,
}
#[derive(Debug)]
pub struct Client {
    pub thread_id: f64,
    pub client_id: String,
    pub subscriptions: Vec<Subscription>
}
#[derive(Debug)]
pub struct Subscription {
    topic_title: String,
    messages: Vec<SubscriptionMessage>,
}
#[derive(Debug)]
pub struct SubscriptionMessage {
    message: String,
}

impl BrokerState {
    // New function
    pub fn new() -> Self {
        BrokerState {
            clients: Vec::new(),
        }
    }
}

impl Client {
    pub fn new(thread_id: f64) -> Self {
        Client {
            thread_id: thread_id,
            client_id: String::new(),
            subscriptions: Vec::new(),
        }
    }
}

impl Subscription {
    pub fn new(topic: String) -> Self {
        Subscription {
            topic_title: topic,
            messages: Vec::new(),
        }
    }
}

impl SubscriptionMessage {
    pub fn new() -> Self {
        SubscriptionMessage {
            message: String::new(),
        }
    }
}

