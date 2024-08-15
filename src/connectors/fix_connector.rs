use env_logger::fmt::Timestamp;
use tokio::sync::mpsc;
use log::error;

pub struct FixMessage {
    pub symbol: String,
    pub price: f64,
    pub timestamp: String,
}

pub async fn fix_connector(fix_server_url: &str, sender: mpsc::Sender<FixMessage>) -> Result<(), Box<dyn std::error::Error>>{
    let mut fix_session = connect_to_fix_server(fix_server_url).await?;

    while let Some(fix_message) = fix_session.next().await {
        match fix_message {
            Ok(message) => {
                let parsed_message = parse_fix_message(message)?;
                if let Err(e) = sender.send(parsed_message).await {
                    error!("Failed to send FIX data: {}", e);
                }
            },
            Err(e) =>{
                error!("FIX session error: {}",e);
            }
        }
    }
    Ok(())
}

async fn connect_to_fix_server(_fix_server_url: &str) -> Result<FixSession,Box<dyn std::error::Error>> {
    Ok(FixSession {})
}

fn parse_fix_message(_raw_message: RawFixMessage) -> Result<FixMessage,Box<dyn std::error::Error>>{
    Ok(FixMessage{
        symbol: "AAPL".to_string(),
        price: 150.0,
        timestamp: "2024-08-15T12:34:56Z".to_string(),
    })
}

struct FixSession;

impl FixSession{
    async fn next(&mut self) -> Option<Result<RawFixMessage, Box<dyn std::error::Error>>>{
        None
    }
}

struct RawFixMessage{
    symbol: String,
    price: f64,
    timestamp: String,
}