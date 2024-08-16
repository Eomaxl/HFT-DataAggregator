// src/api/fix_handler.rs

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::connectors::fix_connector::session::FixSession;
use crate::connectors::fix_connector::message::FixMessage;

#[derive(Deserialize)]
struct FixMessageInput {
    message: String,
}

#[derive(Serialize)]
struct FixMessageOutput {
    status: String,
    message: Option<String>,
}

async fn send_fix_message(
    session: web::Data<Arc<FixSession>>,
    input: web::Json<FixMessageInput>,
) -> impl Responder {
    match FixMessage::parse(&input.message) {
        Some(fix_msg) => {
            let mut session = Arc::clone(&session);
            if let Err(_) = session.send_message(fix_msg).await {
                return HttpResponse::Ok().json(FixMessageOutput {
                    status: "Failed".into(),
                    message: None,
                });
            }
            HttpResponse::Ok().json(FixMessageOutput {
                status: "Sent".into(),
                message: Some(input.message.clone()),
            })
        }
        None => HttpResponse::Ok().json(FixMessageOutput {
            status: "Failed".into(),
            message: None,
        }),
    }
}

async fn receive_fix_message(
    session: web::Data<Arc<FixSession>>,
) -> impl Responder {
    let mut session = Arc::clone(&session);
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/fix/send")
            .route(web::post().to(send_fix_message)),
    )
        .service(
            web::resource("/fix/receive")
                .route(web::get().to(receive_fix_message)),
        );
}
