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
            0x01 => Some(MessageType::Connect),
            0x02 => Some(MessageType::Connack),
            0x03 => Some(MessageType::Publish),
            0x04 => Some(MessageType::Puback),
            0x05 => Some(MessageType::Pubrec),
            0x06 => Some(MessageType::Pubrel),
            0x07 => Some(MessageType::Pubcomp),
            0x08 => Some(MessageType::Subscribe),
            0x09 => Some(MessageType::Suback),
            0x0A => Some(MessageType::Unsubscribe),
            0x0B => Some(MessageType::Unsuback),
            0x0C => Some(MessageType::Pingreq),
            0x0D => Some(MessageType::Pingresp),
            0x0E => Some(MessageType::Disconnect),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            MessageType::Connect => 0x01,
            MessageType::Connack => 0x02,
            MessageType::Publish => 0x03,
            MessageType::Puback => 0x04,
            MessageType::Pubrec => 0x05,
            MessageType::Pubrel => 0x06,
            MessageType::Pubcomp => 0x07,
            MessageType::Subscribe => 0x08,
            MessageType::Suback => 0x09,
            MessageType::Unsubscribe => 0x0A,
            MessageType::Unsuback => 0x0B,
            MessageType::Pingreq => 0x0C,
            MessageType::Pingresp => 0x0D,
            MessageType::Disconnect => 0x0E,
        }
    }
}
