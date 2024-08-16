// src/connectors/rest_connector/client.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct TwelveDataAPI {
    pub(crate) client: Client,
    pub(crate) api_key: String,
}

impl TwelveDataAPI {
    pub fn new(api_key: &str) -> Self {
        TwelveDataAPI {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
    pub country: String,
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSeriesMeta {
    pub symbol: String,
    pub interval: String,
    pub currency: String,
    pub exchange_timezone: String,
    pub exchange: String,
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSeriesValue {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSeriesResponse {
    pub meta: TimeSeriesMeta,
    pub values: Vec<TimeSeriesValue>,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookLevel {
    pub price: String,
    pub quantity: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookResponse {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub status: String,
}
