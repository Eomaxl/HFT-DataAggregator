use crate::processing::data_processing::StockData;
use log::error;

pub async fn dispatch_data(data: Vec<StockData>) ->Result<(),Box<dyn std::error::Error>>{
    for stock_data in data {
        if let Err(e) = send_to_trading_engine(&stock_data).await {
            error!("Failed to dispatch data: {}", e);
        }
    }
    Ok(())
}

async fn send_to_trading_engine(data: &StockData) -> Result<(), Box<dyn std::error::Error>> {
    println!("Dispatching data: {:?}", data);
    Ok(())
}