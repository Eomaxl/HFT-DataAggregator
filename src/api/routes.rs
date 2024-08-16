// src/api/routes.rs

use actix_web::{web, HttpResponse, Responder};
use crate::connectors::rest_connector::client::TwelveDataAPI;
use std::sync::Arc;
use serde::Deserialize;

async fn get_stocks(api: web::Data<Arc<TwelveDataAPI>>) -> impl Responder {
    match api.get_stock_list().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_time_series(
    api: web::Data<Arc<TwelveDataAPI>>,
    query: web::Query<TimeSeriesQuery>,
) -> impl Responder {
    match api
        .get_time_series(
            query.symbols.clone(),
            &query.interval,
            query.start_date.as_deref(),
            query.end_date.as_deref(),
            query.outputsize.as_deref(),
        )
        .await
    {
        Ok(time_series) => HttpResponse::Ok().json(time_series),
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
            .route(web::get().to(get_stocks)),
    )
        .service(
            web::resource("/time_series")
                .route(web::get().to(get_time_series)),
        );
}
