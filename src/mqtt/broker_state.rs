use std::net::{TcpListener, TcpStream};

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
    pub tcp_stream: TcpStream,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub topic_title: String,
    pub sub_qos: u8,
    pub messages: Vec<SubscriptionMessage>,
}

#[derive(Debug, Clone)]
pub struct SubscriptionMessage {
    pub packet_identifier: u16,
    pub message: String,
    pub pub_qos: u8,
    pub message_state: MessageState,
    pub last_updated: OffsetDateTime,       // Timestamp for last state/message update
    pub retry_count: u8,                    // Number of times the message has been retried
}

#[derive(Debug, Clone)]
pub enum MessageState {
    // Default state
    None,
    
    // Publish states
    PublishSent,
    PublishAcknowledged,
    PublishReceived,
    PublishReleased,

    // Subscribe states
    MessageAcknowledged,
    MessageReceived,
    MessageCompleted,
    MessageUnsuccessful,
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
    pub fn new(thread_id: f64, 
        client_id:String,
        will_topic:String,
        will_text:String,
        will_retain: bool,
        will_qos: u8,
        clean_session: bool,
        keep_alive_seconds: usize,
        subscriptions: Vec<Subscription>,
        cancellation_requested: bool, 
        tcp_stream: TcpStream)
        -> Self {
            let now = OffsetDateTime::now_utc();
            Client {
                thread_id: thread_id,
                client_id: client_id,
                will_topic: will_topic,
                will_text: will_text,
                will_retain: will_retain,
                will_qos: will_qos,
                clean_session: clean_session,
                keep_alive_seconds: keep_alive_seconds,
                last_connection: PrimitiveDateTime::new(now.date(), now.time()),
                subscriptions: subscriptions,
                cancellation_requested: cancellation_requested,
                tcp_stream: tcp_stream,
            }
        }
        
        pub fn update_message_state(&mut self, packet_identifier: u16, new_state: MessageState) {
            for subscription in &mut self.subscriptions {
                for message in &mut subscription.messages {
                    if message.packet_identifier == packet_identifier {
                        message.update_state(new_state.clone());
                    }
                }
            }
        }
        
        pub fn remove_message(&mut self, packet_identifier: u16) {
            for subscription in &mut self.subscriptions {
                subscription.messages.retain(|message| message.packet_identifier != packet_identifier);
            }
        }
    }
    impl Clone for Client {
        fn clone(&self) -> Self {
            Client {
                thread_id: self.thread_id,
                cancellation_requested: self.cancellation_requested,
                subscriptions: self.subscriptions.clone(),
                last_connection: self.last_connection,
                client_id: self.client_id.clone(),
                will_topic: self.will_topic.clone(),
                will_text: self.will_text.clone(),
                will_retain: self.will_retain,
                will_qos: self.will_qos,
                clean_session: self.clean_session,
                keep_alive_seconds: self.keep_alive_seconds,
                tcp_stream: self.tcp_stream.try_clone().expect("Failed to clone TcpStream"),
            }
        }
    }
    
    impl Subscription {
        pub fn new(topic: String, qos: u8) -> Self {
        Subscription {
            topic_title: topic,
            sub_qos: qos,
            messages: Vec::new(),
        }
    }
}

impl SubscriptionMessage {
    pub fn new(message: String, qos: u8, message_state: MessageState, packet_identifier: u16) -> Self {
        SubscriptionMessage {
            packet_identifier: packet_identifier,
            message: message,
            pub_qos: qos,
            message_state: message_state,
            last_updated: OffsetDateTime::now_utc(),
            retry_count: 0,
        }
    }

    pub fn update_state(&mut self, new_state: MessageState) {
        self.message_state = new_state;
        self.last_updated = OffsetDateTime::now_utc();
        //self.retry_count = 0; // Reset the retry count
    }

    pub fn add_retry(&mut self) {
        self.retry_count += 1;
        self.last_updated = OffsetDateTime::now_utc();
    }
}

