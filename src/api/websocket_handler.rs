// src/api/websocket_handler.rs

use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use serde::Deserialize;
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::connectors::websocket_connector::websocket_connector;
use crate::connectors::rest_connector::client::TwelveDataAPI;

pub struct WebSocketSession {
    tx: broadcast::Sender<String>,
    rx: broadcast::Receiver<String>,
}

impl WebSocketSession {
    pub fn new(tx: Sender<dyn Message<Result=()>>, rx: Receiver<dyn Message<Result=()>>) -> Self {
        WebSocketSession { tx, rx }
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let rx = self.rx?;
        let addr = ctx.address();

        // Spawn a task to handle incoming messages from the broadcast channel
        tokio::spawn(async move {
            let mut rx = rx;
            while let Ok(msg) = rx.recv().await {
                addr.do_send(ClientMessage(msg))?;
            }
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub String);

impl Handler<ClientMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Handle incoming text message from client
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => {
                // Handle incoming binary message from client
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

#[derive(Deserialize)]
pub struct WebSocketQuery {
    symbols: Vec<String>,
}

/// WebSocket route handler
pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    api: web::Data<Arc<TwelveDataAPI>>,
    query: web::Query<WebSocketQuery>,
) -> Result<HttpResponse, Error> {
    let (tx, _) = broadcast::channel(100);
    let rx = tx.subscribe();

    let symbols = query.symbols.clone();
    let api_clone = api.clone();

    // Spawn the websocket connector to handle messages from Twelve Data
    tokio::spawn(async move {
        if let Err(e) = websocket_connector(&api_clone.api_key, symbols, tx.clone()).await {
            eprintln!("Error in WebSocket connector: {}", e);
        }
    });

    // Start the WebSocket session
    ws::start(WebSocketSession::new(tx, rx), &req, stream)
}

/// Function to configure the WebSocket route
pub fn configure_websocket_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ws")
            .route(web::get().to(websocket_route)),
    );
}
