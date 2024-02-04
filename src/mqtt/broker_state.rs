use std::string;

#[derive(Debug)]
pub struct BrokerState {
    pub clients: Vec<Client>,
}
#[derive(Debug)]
pub struct Client {
    pub thread_id: f64,
    pub cancellation_requested: bool,
    pub client_id: String,
    pub subscriptions: Vec<Subscription>
}
#[derive(Debug)]
pub struct Subscription {
    pub topic_title: String,
    pub messages: Vec<SubscriptionMessage>,
}
#[derive(Debug, Clone)]
pub struct SubscriptionMessage {
    pub message: String,
    pub message_sent: bool,
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
            cancellation_requested: false,
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
    pub fn new(message: String) -> Self {
        SubscriptionMessage {
            message: message,
            message_sent: false
        }
    }
}
