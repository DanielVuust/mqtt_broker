// MQTT messagetypes

#[derive(Debug)]
pub enum MessageType {
    Connect,
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
}

impl MessageType {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x10 => Some(MessageType::Connect),
            0x20 => Some(MessageType::Connack),
            0x30 => Some(MessageType::Publish),
            0x40 => Some(MessageType::Puback),
            0x50 => Some(MessageType::Pubrec),
            0x60 => Some(MessageType::Pubrel),
            0x70 => Some(MessageType::Pubcomp),
            0x80 => Some(MessageType::Subscribe),
            0x90 => Some(MessageType::Suback),
            0xA0 => Some(MessageType::Unsubscribe),
            0xB0 => Some(MessageType::Unsuback),
            0xC0 => Some(MessageType::Pingreq),
            0xD0 => Some(MessageType::Pingresp),
            0xE0 => Some(MessageType::Disconnect),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            MessageType::Connect => 0x10,
            MessageType::Connack => 0x20,
            MessageType::Publish => 0x30,
            MessageType::Puback => 0x40,
            MessageType::Pubrec => 0x50,
            MessageType::Pubrel => 0x60,
            MessageType::Pubcomp => 0x70,
            MessageType::Subscribe => 0x80,
            MessageType::Suback => 0x90,
            MessageType::Unsubscribe => 0xA0,
            MessageType::Unsuback => 0xB0,
            MessageType::Pingreq => 0xC0,
            MessageType::Pingresp => 0xD0,
            MessageType::Disconnect => 0xE0,
        }
    }
}
