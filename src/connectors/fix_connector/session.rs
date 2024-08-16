// src/connectors/fix_connector/session.rs

use tokio::sync::mpsc::{Sender, Receiver};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use std::error::Error;
use std::fmt;
use crate::connectors::fix_connector::message::FixMessage;

pub struct FixSession {
    sender: Sender<String>,
    receiver: Receiver<String>,
    sequence_number: u64,
    stream: Option<TlsStream<TcpStream>>,
}

impl FixSession {
    pub fn new(sender: Sender<String>, receiver: Receiver<String>) -> Self {
        FixSession {
            sender,
            receiver,
            sequence_number: 1,
            stream: None,
        }
    }

    pub async fn start_session(&mut self, address: &str) -> Result<(), Box<dyn Error>> {
        let stream = connect_to_fix_server(address).await?;
        self.stream = Some(stream);

        // Example: Send a Logon message at the start of the session
        let logon_message = self.create_logon_message();
        self.send_message(logon_message).await?;

        Ok(())
    }

    pub fn create_logon_message(&self) -> FixMessage {
        let mut fields = std::collections::HashMap::new();
        fields.insert("35".to_string(), "A".to_string()); // MsgType=Logon
        fields.insert("34".to_string(), self.sequence_number.to_string()); // MsgSeqNum
        FixMessage::new("A", fields)
    }

    pub async fn send_message(&mut self, msg: FixMessage) -> Result<(), Box<dyn Error>> {
        self.sequence_number += 1;
        let msg_str = msg.to_string(); // Convert FIX message to string

        if let Some(ref mut stream) = self.stream {
            stream.write_all(msg_str.as_bytes()).await?;
        }

        // Optionally, send the message over the mpsc channel if needed
        self.sender.send(msg_str).await?;

        Ok(())
    }

    pub async fn receive_message(&mut self) -> Option<String> {
        if let Some(ref mut stream) = self.stream {
            let mut buf = vec![0; 1024];
            return match stream.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    Some(String::from_utf8_lossy(&buf[..n]).to_string())
                }
                _ => None,
            }
        }
        None
    }

    pub fn get_next_seq_num(&self) -> u64 {
        self.sequence_number
    }
}

async fn connect_to_fix_server(address: &str) -> Result<TlsStream<TcpStream>, Box<dyn Error>> {
    let stream = TcpStream::connect(address).await?;
    let native_tls_connector = native_tls::TlsConnector::new()?;
    let tls_connector = tokio_native_tls::TlsConnector::from(native_tls_connector);
    let tls_stream = tls_connector.connect("example.com", stream).await?;
    Ok(tls_stream)
}

// Implementing Display trait for FixMessage
impl fmt::Display for FixMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format FixMessage fields as needed
        write!(f, "FIX message with fields: {:?}", self.fields)
    }
}
