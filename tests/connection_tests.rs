
mod common;
use crate::common::mqtt_commands_utils::{connack_vec, connect_vec,};
use crate::common::test_utils::start_test_broker;
use crate::common::test_client::TestClient;

// Use this command: cargo test --test connection_tests

#[test]
fn connect_with_no_falgs_test() {

    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Creates a connect request 
    let connect_request = connect_vec(0, 60, "test".to_string());

    //Sends the connect request to the borker
    client.send_message(&connect_request).unwrap();

    //Reads the response from the server
    let response = client.read_response().unwrap();

    // Validates if the response is a conack
    assert_eq!(response, connack_vec());

    // Closes the connection
    client.close().unwrap();
}

#[test]
fn connect_with_long_client_indetifier(){
    //Starts the the broker used for tests.
    start_test_broker();

    //Creates a client and connects it to the broker.
    let mut client = TestClient::connect("127.0.0.1:7878").unwrap();

    //Creates a connect request 
    let connect_request = connect_vec(0, 60, 
        "Lorem ipsum, or lipsum as it is sometimes known, is dummy text used in laying out print, graphic or web designs. The passage is attributed to an unknown typesetter in the 15th century who is thought to have scrambled parts of Cicero's De Finibus Bonorum et Malorum for use in a type specimen book. It usually begins with:

        “Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.”
        The purpose of lorem ipsum is to create a natural looking block of text (sentence, paragraph, page, etc.) that doesn't distract from the layout. A practice not without controversy, laying out pages with meaningless filler text can be very useful when the focus is meant to be on design, not content.
        
        The passage experienced a surge in popularity during the 1960s when Letraset used it on their dry-transfer sheets, and again during the 90s as desktop publishers bundled the text with their software. Today it's seen all around the web; on templates, websites, and stock designs. Use our generator to get your own, or read on for the authoritative history of lorem ipsum".to_string());

    //Sends the connect request to the borker
    client.send_message(&connect_request).unwrap();

    //Reads the response from the server
    let response = client.read_response().unwrap();

    // Validates if the response is a conack
    assert_eq!(response, connack_vec());

    // Closes the connection
    client.close().unwrap();
}


