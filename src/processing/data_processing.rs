use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use env_logger::fmt::Timestamp;
use log::error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StockData {
    pub symbol: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}

pub fn parse_raw_data(raw_data: &str) -> Result<StockData,Box<dyn std::error::Error>>{
    let parsed_data: StockData = serde_json::from_str(raw_data)?;
    Ok(parsed_data)
}

pub fn validate_data(data:StockData) -> bool {
    if data.symbol.is_empty() || data.price <= 0.0 {
        error!("Invalid data: {:?}", data);
        return false;
    }
    true
}

pub fn transform_data(mut data: StockData) -> StockData {
    data.timestamp = data.timestamp.with_timezone(&Utc);
    data
}