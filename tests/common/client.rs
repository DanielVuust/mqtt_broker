use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct TestClient {
    pub stream: TcpStream,
}

impl TestClient{
    pub fn connect(addr: &str) -> TestClient {
        let stream = TcpStream::connect(addr).unwrap();
        stream.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
        stream.set_write_timeout(Some(Duration::from_millis(500))).unwrap();
    }
}