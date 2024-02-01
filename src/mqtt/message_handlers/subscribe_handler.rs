

//TODO implment handeling of multiple 
pub fn handle_subscribe(buffer: &[u8], mut current_buffer_index: usize) -> String{
    
    let topic_length = buffer[current_buffer_index] as usize * 256 as usize + buffer[current_buffer_index + 1] as usize;
    current_buffer_index += 2;
    println!("{}", topic_length);

    let mut topic = "".to_string();
    for (index) in current_buffer_index..current_buffer_index+topic_length{
        topic.push(buffer[index] as char);
    }
    current_buffer_index += topic_length;
    let topic_qos: u8 = buffer[current_buffer_index];
    current_buffer_index += 1;

    topic
}