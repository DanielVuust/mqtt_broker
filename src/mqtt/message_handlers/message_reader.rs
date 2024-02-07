

//TODO implement byte shifting.
//Currently it only supports a max length of 255
pub fn read_package_length(buffer: &[u8], mut index: usize) -> (usize, usize){
    //Plus 2 as the length number is not included the package identifier package(index 0) and the length bytes (index 1-4). this should be changed when adding support to packges more than 255.
    let length = buffer[index] + 2;
    index += 1;
    (length as usize, index)
}

pub fn read_utf_8_string_with_length_bytes(buffer: &[u8], mut index: usize) -> (String, usize){

    let string_length = buffer[index] as usize * 256 as usize + buffer[index + 1] as usize;
    index += 2;

    let mut result = "".to_string();
    for index in index..index+string_length{
        result.push(buffer[index] as char);
    }
    index += string_length;
    (result, index)
}

pub fn read_utf_8_string_with_end_index(buffer: &[u8], mut current_index: usize, end_index: usize) -> (String, usize){
    println!("current index{:?}", current_index);

    let mut result = "".to_string();
    for index in current_index..=end_index{
        result.push(buffer[index] as char);
    }
    current_index = end_index;
    (result, current_index)
}