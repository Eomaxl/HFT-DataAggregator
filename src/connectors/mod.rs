pub(crate) mod websocket_connector;
pub mod rest_connector {
    pub mod client;
    pub mod models;
    pub mod stock_api;
    pub mod time_series_api;
    pub mod order_book_api;
}
pub mod fix_connector {
    pub mod connection;
    pub mod error_handling;
    pub mod models;
    pub mod session;
    pub mod sequence;
    pub mod message;
}