


pub fn connect_vec(connect_falgs_byte: u8, keep_alive: u16, client_identifier: String,) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.append(&mut vec![0x10]); // MQTT Control Packet type for CONNECT
    bytes.append(&mut vec![0x00]); // Remaining Length (will be calculated at the end of fuction)
    bytes.append(&mut vec![0x00, 0x04]); // Protocol Name Length 
    bytes.append(&mut vec![0x4D, 0x51, 0x54, 0x54]); // Protocol Name MQTT 'Q'
    bytes.append(&mut vec![0x04]); // Protocol Level (0x04 for MQTT 3.1.1)
    bytes.append(&mut vec![connect_falgs_byte]); // Protocol Level (0x04 for MQTT 3.1.1)
    
    let keep_alive_bytes = convert_u16_to_two_u8s_be(keep_alive);
    bytes.append(&mut vec![keep_alive_bytes[1], keep_alive_bytes[0]]); // Keep Alive 
    
    let client_identifier_length = convert_u16_to_two_u8s_be(client_identifier.len() as u16);
    bytes.append(&mut vec![client_identifier_length[1], client_identifier_length[0]]); // Client identifier length
    bytes.append(&mut client_identifier.as_bytes().to_vec()); //Client identifier

    bytes[1] = bytes.len() as u8 - 2;
    bytes
}

pub fn connack_vec() -> Vec<u8> {
    // Minimal CONNACK packet
    vec![0x20, 0x02, 0x00, 0x00]
}
fn convert_u16_to_two_u8s_be(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}
pub fn subscribe_vec(topic_names: Vec<String>, requested_qos: Vec<u8>, packet_identifier: u16) -> Vec<u8>{
    let mut bytes = vec![];
    bytes.append(&mut vec![0x80]); // MQTT Control Packet type for subscribe
    bytes.append(&mut vec![0x00]); // Remaining Length (will be calculated at the end of fuction)

    bytes.append(&mut convert_u16_to_two_u8s_be(packet_identifier).to_vec()); //Packet Identifier

    for (index, topic_name) in topic_names.iter().enumerate()
    {
        bytes.append(&mut convert_u16_to_two_u8s_be(topic_name.len() as u16).to_vec()); // Client identifier length
        bytes.append(&mut topic_name.as_bytes().to_vec()); 

        bytes.append(&mut vec![requested_qos[index]]);
    }
    bytes[1] = bytes.len() as u8 - 2; 

    bytes
}

pub fn suback_vec(topic_results: Vec<u8>, packet_identifier: u16) -> Vec<u8>{
    let mut bytes = vec![];
    bytes.append(&mut vec![0x90]); // MQTT Control Packet type for subscribe
    bytes.append(&mut vec![0x00]); // Remaining Length (will be calculated at the end of fuction)

    bytes.append(&mut convert_u16_to_two_u8s_be(packet_identifier).to_vec()); //Packet Identifier

    for (_index, topic_result) in topic_results.iter().enumerate()
    {
        bytes.append(&mut vec![*topic_result]);
    }
    bytes[1] = bytes.len() as u8 - 2; 

    bytes
}

pub fn publish_vec(topic_name: String, qos: u8, message: String, packet_identifier: u16) -> Vec<u8>{

    let mut bytes = vec![];
    let mut control_packet_byte = 48; //48 = publish 
    control_packet_byte += qos;
    bytes.append(&mut vec![control_packet_byte]); // MQTT Control Packet type for publish without DUP = false, qos from parameter and retain = false
    bytes.append(&mut vec![0x00]); // Remaining Length (will be calculated at the end of fuction)

    bytes.append(&mut convert_u16_to_two_u8s_be(topic_name.len() as u16).to_vec()); //Packet Identifier

    bytes.append(&mut topic_name.as_bytes().to_vec());

    //If qos is more then 0 then the package should include a packet_identifier.
    if(qos > 0){
        bytes.append(&mut convert_u16_to_two_u8s_be(packet_identifier).to_vec()); //Packet Identifier
    }  
    bytes.append(&mut message.as_bytes().to_vec());


    bytes[1] = bytes.len()as u8 - 2;
    bytes
}
