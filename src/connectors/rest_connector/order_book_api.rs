// src/connectors/rest_connector/order_book_api.rs

use super::client::TwelveDataAPI;
use super::models::OrderBookResponse;
use std::error::Error;

impl TwelveDataAPI {
    /// Fetch market depth data for a given symbol
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
