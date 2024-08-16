// src/connectors/rest_connector/time_series_api.rs

use super::client::TwelveDataAPI;
use super::models::TimeSeriesResponse;
use std::error::Error;

impl TwelveDataAPI {
    /// Fetch historical time series data for the specified symbols and parameters
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
}
