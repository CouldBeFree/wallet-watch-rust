use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;

pub async fn db_connect() -> Database {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("Failed to parse options");
    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");
    client.database("wallet_watch")
}

pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("rust_axum_users_db"))
}
