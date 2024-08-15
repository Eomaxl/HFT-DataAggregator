use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use futures::StreamExt;
use log::error;

pub async fn websocket_connector(ws_url: &str, sender: mpsc::Sender<String>)->Result<(),Box<dyn std::error::Error>>{
    let (ws_stream, _) = connect_async(ws_url).await?;
    let (_,mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if let Ok(text) = msg.to_text(){
                    if let Err(e) = sender.send(text.to_string()).await{
                        error!("Failed to send Websocket data : {}",e);
                    }
                }
            },
            Err(e) => {
                error!("WebSocket error: {}", e);
            }
        }
    }
    Ok(())
}