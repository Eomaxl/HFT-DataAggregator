use tokio::sync::mpsc;
use reqwest::Client;
use log::info;
use std::time::Instant;

pub async fn rest_connector(api_url: &str, api_key: &str, sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>>{
    let client =  Client::new();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        interval.tick().await;
        let start = Instant::now();
        let response = client.get(api_url).header("Authorization",format!("Bearer {}",api_key)).send().await?.text().await?;
        let duration = start.elapsed();

        info!("Fetched data  in {:?}",duration);

        if let Err(e) = sender.send(response).await {
            log::error!("Failed to send data : {}",e);
        }
    }
}