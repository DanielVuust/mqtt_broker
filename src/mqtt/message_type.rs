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
            0x10..=0x1F => Some(MessageType::Connect),
            0x20..=0x2F => Some(MessageType::Connack),
            0x30..=0x3F => Some(MessageType::Publish),
            0x40..=0x4F => Some(MessageType::Puback),
            0x50..=0x5F => Some(MessageType::Pubrec),
            0x60..=0x6F => Some(MessageType::Pubrel),
            0x70..=0x7F => Some(MessageType::Pubcomp),
            0x80..=0x8F=> Some(MessageType::Subscribe),
            0x90..=0x9F => Some(MessageType::Suback),
            0xA0..=0xAF => Some(MessageType::Unsubscribe),
            0xB0..=0xBF => Some(MessageType::Unsuback),
            0xC0..=0xCF => Some(MessageType::Pingreq),
            0xD0..=0xDF => Some(MessageType::Pingresp),
            0xE0..=0xEF => Some(MessageType::Disconnect),
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
