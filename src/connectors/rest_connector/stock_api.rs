// src/connectors/rest_connector/stock_api.rs

use super::client::TwelveDataAPI;
use super::models::Stock;
use std::error::Error;

impl TwelveDataAPI {
    /// Fetch the list of stocks available through the Twelve Data API
    pub async fn get_stock_list(&self) -> Result<Vec<Stock>, Box<dyn Error>> {
        let url = format!("https://api.twelvedata.com/stocks?apikey={}", self.api_key);
        let response = self.client.get(&url).send().await?.json::<Vec<Stock>>().await?;
        Ok(response)
    }
}
