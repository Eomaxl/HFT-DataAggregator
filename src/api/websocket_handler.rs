// src/api/websocket_handler.rs

use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message;
use crate::connectors::websocket_connector::websocket_connector;
use crate::connectors::rest_connector::client::TwelveDataAPI;

pub struct WebSocketSession {
    tx: broadcast::Sender<Message>,
    rx: broadcast::Receiver<Message>,
}

impl WebSocketSession {
    pub fn new(tx: broadcast::Sender<Message>, rx: broadcast::Receiver<Message>) -> Self {
        WebSocketSession { tx, rx }
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut rx = self.rx.resubscribe();  // Use resubscribe to get a new receiver
        let addr = ctx.address();

        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                addr.do_send(ClientMessage(msg));
            }
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub Message);

impl Handler<ClientMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        if let Ok(text) = msg.0.to_text() {
            ctx.text(text);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => {
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

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    api: web::Data<Arc<TwelveDataAPI>>,
    query: web::Query<WebSocketQuery>,
) -> impl Responder {
    let (tx, _) = broadcast::channel(100);
    let rx = tx.subscribe();

    let symbols = query.symbols.clone();
    let api_clone = api.clone();
    let tx_clone = tx.clone();  // Clone tx before moving it into the closure

    tokio::spawn(async move {
        if let Err(e) = websocket_connector(&api_clone.api_key, symbols, tx_clone).await {
            eprintln!("Error in WebSocket connector: {}", e);
        }
    });

    ws::start(WebSocketSession::new(tx, rx), &req, stream)
}
