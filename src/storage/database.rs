use sqlx::postgres::PgPoolOptions;
use sqlx::{query, Pool, Postgres};
use crate::processing::data_processing::StockData;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .expect("Failed to create pool.");

        Database { pool }
    }

    // pub async fn store_data(&self, data: &StockData) -> Result<(), sqlx::Error> {
    //     query!(
    //         "INSERT INTO stock_data (symbol, price, timestamp) VALUES ($1, $2, $3)",
    //         data.symbol,
    //         data.price,
    //         data.timestamp
    //     )
    //         .execute(&self.pool)
    //         .await?;
    //
    //     Ok(())
    // }
}
