mod blockchain;
pub mod cli;
pub mod models;
pub mod rocket;
pub mod schema;

use blockchain::{connection::*, data::*};
pub use cli::*;
use diesel::{QueryDsl, RunQueryDsl};
use models::Blockchain;
use rocket::api::*;

#[tokio::main]
async fn main() {
    println!("🚀 Launching into the Blockchain Universe! 🌐");
    println!("🛠️  Initializing the Rocket Server...\n");

    match main_menu() {
        1 => show_data_cli().await,
        2 => rocket_launch().await,
        _ => println!("❌ Invalid choice. Restart the program."),
    }
}

// Check the Data and store in the Blockchain
async fn store_blockchain(endpoint: String) -> Result<Blockchain,diesel::result::Error>  {
    println!("💾 Preparing to store blockchain data...");

    // Check if endpoint starts with "ws"
    if !endpoint.starts_with("ws") {
        println!("⚠️ WebSocket endpoint must start with 'ws://' or 'wss://'.");
        return Err(diesel::result::Error::NotFound);
    }

    match establish_ws_connection(&endpoint).await {
        Ok(client) => {
            println!("✅ Connection Established! 🎉");
        
            let name = get_blockchain_name(client).await;
            let validators = current_validators(&endpoint).await;

            let hex_validators: Vec<String> = validators
                .iter()
                .map(|v| format!("0x{}", hex::encode(v.0)))
                .inspect(|v| println!("Validator: {:?}", v))
                .collect();

            println!(
                "Store this in the database? Type '1' to store or any other key word to exit:"
            );

             store_db(&name.unwrap(), hex_validators, validators.len() as i32)
            
        }
       Err(_)=> Err(diesel::result::Error::NotInTransaction)
    }
}

fn delete_blockchain(id: i32) {
    let mut connection = establish_connection();

    match diesel::delete(schema::blockchain_info::table.find(id)).execute(&mut connection) {
        Ok(_) => println!("✅ Successfully deleted blockchain with ID {}.", id),
        Err(e) => println!("❌ Error deleting blockchain: {:?}", e),
    }
}
