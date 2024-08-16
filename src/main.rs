// src/main.rs

mod connectors;
mod api;
mod storage;
mod processing;
mod monitoring;
mod dispatch;

use actix_web::{web, App, HttpServer};
use tokio::sync::Mutex;
use std::sync::Arc;
use env_logger;
use actix_web::middleware::Logger;
use crate::api::fix_handler::{send_fix_message, receive_fix_message};
use crate::api::websocket_handler::websocket_route;
use crate::connectors::rest_connector::client::TwelveDataAPI;
use crate::connectors::fix_connector::session::FixSession;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the Twelve Data API client
  //  let api_key = std::env::var("API_KEY").expect("API_KEY not set in .env file");
    let api_key = "7fbabfd1505f48d5ab629c0a3c0d9ee5";
    let twelve_data_api = Arc::new(TwelveDataAPI::new(&api_key));

    // Initialize the FIX session with channels for sending and receiving messages
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let fix_session = Arc::new(Mutex::new(FixSession::new(tx, rx)));

    // Start the FIX session (assuming FIX server address is provided)
    let fix_session_clone = fix_session.clone();
    let fix_server_address = "127.0.0.1";
    let fix_server_port = 12345;
    tokio::spawn(async move {
        let address = format!("{}:{}", fix_server_address, fix_server_port);
        if let Err(e) = fix_session_clone.lock().await.start_session(&address).await {
            eprintln!("Error in FIX session: {}", e);
        }
    });

    // Start the Actix-web server and configure the routes
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(fix_session.clone()))
            .app_data(web::Data::new(twelve_data_api.clone()))
            .service(web::resource("/send_fix_message").route(web::post().to(send_fix_message)))
            .service(web::resource("/receive_fix_message").route(web::get().to(receive_fix_message)))
            .service(web::resource("/ws").route(web::get().to(websocket_route)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
