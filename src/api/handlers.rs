// src/api/handler.rs

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use crate::connectors::rest_connector::client::{TwelveDataAPI, Stock, TimeSeriesResponse, OrderBookResponse};

#[derive(Deserialize)]
pub struct StockQuery {
    symbol: Option<String>,
    exchange: Option<String>,
    mic_code: Option<String>,
    country: Option<String>,
    type_: Option<String>,
}

#[derive(Deserialize)]
pub struct TimeSeriesQuery {
    symbols: Vec<String>,
    interval: String,
    start_date: Option<String>,
    end_date: Option<String>,
    outputsize: Option<String>,
}

#[derive(Deserialize)]
pub struct OrderBookQuery {
    symbol: String,
}

/// Handler to fetch the stock list from Twelve Data API
pub async fn get_stock_list(
    api: web::Data<Arc<TwelveDataAPI>>,
    query: web::Query<StockQuery>,
) -> impl Responder {
    match api.get_stock_list().await {
        Ok(stocks) => {
            // Optionally filter results based on the query parameters
            let filtered_stocks: Vec<Stock> = stocks.into_iter().filter(|stock| {
                (query.symbol.as_ref().map_or(true, |sym| &stock.symbol == sym)) &&
                    (query.exchange.as_ref().map_or(true, |ex| &stock.exchange == ex)) &&
                    (query.mic_code.as_ref().map_or(true, |mic| &stock.mic_code == mic)) &&
                    (query.country.as_ref().map_or(true, |c| &stock.country == c)) &&
                    (query.type_.as_ref().map_or(true, |t| &stock.type_ == t))
            }).collect();
            HttpResponse::Ok().json(filtered_stocks)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Handler to fetch the time series data from Twelve Data API
pub async fn get_time_series(
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

/// Handler to fetch the order book data from Twelve Data API
pub async fn get_order_book(
    api: web::Data<Arc<TwelveDataAPI>>,
    query: web::Query<OrderBookQuery>,
) -> impl Responder {
    match api.get_order_book(&query.symbol).await {
        Ok(order_book) => HttpResponse::Ok().json(order_book),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Function to configure all REST routes for the Twelve Data API
pub fn configure_rest_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stocks")
            .route(web::get().to(get_stock_list)),
    )
        .service(
            web::resource("/time_series")
                .route(web::get().to(get_time_series)),
        )
        .service(
            web::resource("/order_book")
                .route(web::get().to(get_order_book)),
        );
}
