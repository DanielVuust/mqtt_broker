use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct TestClient {
    pub stream: TcpStream,
}

impl TestClient {
    pub fn connect(addr: &str) -> std::io::Result<TestClient> {
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(Duration::from_millis(5000)))?;
        stream.set_write_timeout(Some(Duration::from_millis(5000)))?;
        Ok(Self { stream })
    }

    // Sends a message to the MQTT broker
    pub fn send_message(&mut self, message: &[u8]) -> std::io::Result<()> {
        self.stream.write_all(message)?;
        Ok(())
    }

    // Reads the response from the broker
    pub fn read_response(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buffer = vec![0; 1024];
        let size = self.stream.read(&mut buffer)?;
        buffer.truncate(size);
        Ok(buffer)
    }

    // Close the connection
    pub fn close(&mut self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }
}