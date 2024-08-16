// src/api/routes.rs

use actix_web::{web, HttpResponse, Responder};
use crate::connectors::rest_connector::client::TwelveDataAPI;
use std::sync::Arc;
use serde::Deserialize;
use crate::api::handlers::{get_order_book, get_stock_list};

async fn get_stocks(api: web::Data<Arc<TwelveDataAPI>>) -> impl Responder {
    match api.get_stock_list().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct TimeSeriesQuery {
    symbols: Vec<String>,
    interval: String,
    start_date: Option<String>,
    end_date: Option<String>,
    outputsize: Option<String>,
}

pub fn configure_rest_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stocks")
            .route(web::get().to(get_stock_list)),
    )
        .service(
            web::resource("/time_series")
                .route(web::get().to(crate::api::handlers::get_time_series)),
        )
        .service(
            web::resource("/order_book")
                .route(web::get().to(get_order_book)),
        );
}
