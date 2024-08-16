// src/main.rs

mod connectors;
mod api;
mod storage;
mod processing;
mod monitoring;
mod dispatch;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use crate::connectors::fix_connector::session::FixSession;
use crate::api::fix_handler;
use crate::api::websocket_handler::configure_websocket_routes;
use crate::connectors::rest_connector::client::TwelveDataAPI;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the Twelve Data API client
    let api_key = std::env::var("API_KEY").expect("API_KEY not set in .env file");
    let twelve_data_api = Arc::new(TwelveDataAPI::new(&api_key));

    // Initialize the FIX session with channels for sending and receiving messages
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let fix_session = Arc::new(FixSession::new(tx, rx));

    // Start the FIX session (assuming FIX server address is provided)
    let mut fix_session_clone = fix_session.clone();
    let fix_server_address = "127.0.0.1:12345";
    tokio::spawn(async move {
        if let Err(e) = fix_session_clone.start_session(fix_server_address).await {
            eprintln!("Error in FIX session: {}", e);
        }
    });

    // Start the Actix-web server and configure the routes
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(fix_session.clone()))
            .app_data(web::Data::new(twelve_data_api.clone()))
            .configure(fix_handler::config) // Configure FIX routes
            .configure(configure_websocket_routes)
            .configure(api::routes::configure_rest_routes) // Configure REST API routes for Twelve Data
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
