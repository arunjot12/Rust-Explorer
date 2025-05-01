use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::Blockchain;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use std::env;
use crate::models::NewBlockchain;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Establish WebSocket connection and handle errors
pub async fn establish_ws_connection(endpoint: &str) -> Result<WsClient, String> {
    WsClientBuilder::default()
        .build(endpoint)
        .await
        .map_err(|_| "Failed to connect. Please enter a valid WebSocket endpoint.".to_string())
}

// Store the name in the database
pub fn store_db(blockchain: &str) {
    let new_blockchain = NewBlockchain {
        blockchain_name: blockchain,
    };

    diesel::insert_into(crate::schema::blockchain_info::table)
        .values(&new_blockchain)
        .get_result::<Blockchain>(&mut establish_connection())
        .expect("💥 Error saving new blockchain");

    println!("✅ Blockchain name successfully stored in the database! 🗄️");
}
