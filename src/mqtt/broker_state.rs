use std::{string, time::SystemTime};
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug)]
pub struct BrokerState {
    pub clients: Vec<Client>,
}
#[derive(Debug)]
pub struct Client {
    pub thread_id: f64,
    pub cancellation_requested: bool,
    pub subscriptions: Vec<Subscription>,
    pub last_connection: PrimitiveDateTime,
    
    pub client_id: String,
    pub will_topic: String,
    pub will_text: String,
    pub will_retain: bool,
    pub will_qos: u8,
    pub clean_session: bool,
    pub keep_alive_seconds: usize,
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
        let now = OffsetDateTime::now_utc();
        Client {
            thread_id: thread_id,
            cancellation_requested: false,
            client_id: String::new(),
            subscriptions: Vec::new(),
            last_connection: PrimitiveDateTime::new(now.date(), now.time())
            
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

