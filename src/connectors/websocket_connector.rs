use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use futures::{StreamExt, SinkExt};
use serde_json::json;
use log::{info, error};
use tokio_tungstenite::tungstenite::Message;

pub async fn websocket_connector(
    api_key: &str,
    symbols: Vec<String>,
    tx: broadcast::Sender<Message>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={}", api_key);
    let (mut ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Subscribe to the given symbols
    let subscribe_msg = json!({
        "action": "subscribe",
        "params": {
            "symbols": symbols.join(",")
        }
    }).to_string();

    write.send(Message::Text(subscribe_msg)).await?;

    // Forward incoming messages to broadcast channel
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if let Err(_) = tx.send(msg) {
                    error!("Failed to broadcast WebSocket message");
                }
            },
            Err(e) => {
                error!("WebSocket error: {}", e);
            }
        }
    }

    Ok(())
}
