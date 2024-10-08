use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio_native_tls::TlsStream;
use crate::connectors::fix_connector::models::FixSession;

pub async fn manage_fix_session(
    server: &str,
    port: u16,
    mut session: FixSession,
    mut rx: mpsc::Receiver<String>,
) -> io::Result<()> {
    let address = build_address(server, port);
    let mut stream = connect_to_fix_server(&address).await?;

    loop {
        tokio::select! {
            Some(message) = rx.recv() => {
                handle_outgoing_messages(&message, &mut session, &mut stream).await?;
            }
            result = read_from_stream(&mut stream) => {
                if let Err(e) = result {
                    eprintln!("Error reading from stream: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn build_address(server: &str, port: u16) -> String {
    format!("{}:{}", server, port)
}

async fn handle_outgoing_messages(message: &String, session: &mut FixSession, stream: &mut TlsStream<tokio::net::TcpStream>) -> io::Result<()> {
    // Handle outgoing messages
    let seq_num = session.sequence_manager.get_next_seq_num();
    // Construct the FIX message with the sequence number and send
    stream.write_all(message.as_bytes()).await
}

async fn read_from_stream(stream: &mut TlsStream<tokio::net::TcpStream>) -> io::Result<()> {
    let mut buf = vec![0; 1024];
    match stream.read(&mut buf).await {
        Ok(n) if n == 0 => Ok(()), // Connection closed
        Ok(n) => {
            // Process the incoming FIX message here
            let received_msg = String::from_utf8_lossy(&buf[..n]).to_string();
            println!("Received FIX message: {}", received_msg);
            // You may want to call some handler function here to process the received message
            Ok(())
        }
        Err(e) => Err(e),
    }
}

async fn connect_to_fix_server(address: &str) -> io::Result<TlsStream<tokio::net::TcpStream>> {
    let stream = tokio::net::TcpStream::connect(address).await?;
    let native_tls_connector = native_tls::TlsConnector::new().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    let tls_connector = tokio_native_tls::TlsConnector::from(native_tls_connector);

    let tls_stream = tls_connector.connect("example.com", stream).await.map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    Ok(tls_stream)
}
