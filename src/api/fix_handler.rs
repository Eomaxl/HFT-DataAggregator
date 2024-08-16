// src/api/fix_handler.rs

use actix_web::{web, HttpRequest, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::connectors::fix_connector::session::FixSession;
use crate::connectors::fix_connector::message::FixMessage;

#[derive(Deserialize)]
pub struct FixMessageInput {
    message: String,
}

#[derive(Serialize)]
pub struct FixMessageOutput {
    status: String,
    message: Option<String>,
}

pub async fn send_fix_message(
    req: HttpRequest,
    body: web::Json<FixMessageInput>,
    session: web::Data<Arc<Mutex<FixSession>>>,
) -> impl Responder {
    let fix_message = FixMessage::from_string(body.message.clone());
    let mut session = session.lock().unwrap();
    match session.send_message(fix_message).await {
        Ok(_) => HttpResponse::Ok().json(FixMessageOutput {
            status: "Sent".into(),
            message: Some(body.message.clone()),
        }),
        Err(_) => HttpResponse::Ok().json(FixMessageOutput {
            status: "Failed".into(),
            message: None,
        }),
    }
}

pub async fn receive_fix_message(
    req: HttpRequest,
    session: web::Data<Arc<Mutex<FixSession>>>,
) -> impl Responder {
    let mut session = session.lock().unwrap();
    match session.receive_message().await {
        Some(msg) => HttpResponse::Ok().json(FixMessageOutput {
            status: "Received".into(),
            message: Some(msg),
        }),
        None => HttpResponse::Ok().json(FixMessageOutput {
            status: "No Message".into(),
            message: None,
        }),
    }
}
