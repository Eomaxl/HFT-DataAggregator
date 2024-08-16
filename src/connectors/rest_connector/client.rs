// src/connectors/rest_connector/client.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
    pub country: String,
    pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesMeta {
    pub symbol: String,
    pub interval: String,
    pub currency: String,
    pub exchange_timezone: String,
    pub exchange: String,
    pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesValue {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesResponse {
    pub meta: TimeSeriesMeta,
    pub values: Vec<TimeSeriesValue>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookLevel {
    pub price: String,
    pub quantity: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookResponse {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub status: String,
}

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

    pub async fn get_stock_list(&self) -> Result<Vec<Stock>, Box<dyn Error>> {
        let url = format!("https://api.twelvedata.com/stocks?apikey={}", self.api_key);
        let response = self.client.get(&url).send().await?.json::<Vec<Stock>>().await?;
        Ok(response)
    }

    pub async fn get_time_series(
        &self,
        symbols: Vec<String>,
        interval: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        outputsize: Option<&str>,
    ) -> Result<TimeSeriesResponse, Box<dyn Error>> {
        let mut url = format!(
            "https://api.twelvedata.com/time_series?symbol={}&interval={}&apikey={}",
            symbols.join(","),
            interval,
            self.api_key
        );

        if let Some(start) = start_date {
            url.push_str(&format!("&start_date={}", start));
        }

        if let Some(end) = end_date {
            url.push_str(&format!("&end_date={}", end));
        }

        if let Some(size) = outputsize {
            url.push_str(&format!("&outputsize={}", size));
        }

        let response = self.client.get(&url).send().await?.json::<TimeSeriesResponse>().await?;
        Ok(response)
    }

    pub async fn get_order_book(
        &self,
        symbol: &str,
    ) -> Result<OrderBookResponse, Box<dyn Error>> {
        let url = format!(
            "https://api.twelvedata.com/order_book?symbol={}&apikey={}",
            symbol,
            self.api_key
        );

        let response = self.client.get(&url).send().await?.json::<OrderBookResponse>().await?;
        Ok(response)
    }
}
