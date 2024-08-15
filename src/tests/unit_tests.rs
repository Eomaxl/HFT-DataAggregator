#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::processing::data_processing::{parse_raw_data, transform_data, validate_data, StockData};

    #[test]
    fn test_parse_raw_data() {
        let raw_data = r#"{"symbol": "AAPL", "price": 145.32, "timestamp": "2024-08-15T12:34:56Z"}"#;
        let result = parse_raw_data(raw_data);
        assert!(result.is_ok());
        let stock_data = result.unwrap();
        assert_eq!(stock_data.symbol, "AAPL");
        assert_eq!(stock_data.price, 145.32);
        assert_eq!(stock_data.timestamp.to_string(), "2024-08-15 12:34:56 UTC");
    }

    #[test]
    fn test_validate_data() {
        let valid_data = StockData {
            symbol: "AAPL".to_string(),
            price: 145.32,
            timestamp: Utc::now(),
        };
        assert!(validate_data(valid_data));

        let invalid_data = StockData {
            symbol: "".to_string(),
            price: -10.0,
            timestamp: Utc::now(),
        };
        assert!(!validate_data(invalid_data));
    }

    #[test]
    fn test_transform_data() {
        let data = StockData {
            symbol: "AAPL".to_string(),
            price: 145.32,
            timestamp: Utc::now(),
        };
        let transformed_data = transform_data(data.clone());
        assert_eq!(transformed_data.symbol, data.symbol);
        assert_eq!(transformed_data.price, data.price);
    }
}
