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
        1 => rocket_launch().await,
        2 => show_data_cli().await,
        3 => store_blockchain_detail_cli().await,
        4 => explorer_cli().await(),
        _ => println!("❌ Invalid choice. Restart the program."),
    }
}

async fn store_blockchain_detail_cli() {
    let endpoint = get_websocket_endpoint();
    store_blockchain(endpoint.clone()).await;
    if let Err(e) = process_blocks(&endpoint, true).await {
        eprintln!("❌ Error: {:?}", e);
    };
}

// Check the Data and store in the Blockchain
async fn store_blockchain(endpoint: String) -> Result<(), String> {
    match establish_ws_connection(&endpoint).await {
        Ok(client) => {
            let name = get_blockchain_name(client).await;
            let validators = current_validators(&endpoint).await;

            let hex_validators: Vec<String> = validators
                .iter()
                .map(|v| format!("0x{}", hex::encode(v.0)))
                .inspect(|v| println!("Validator: {:?}", v))
                .collect();

            store_db(&name.unwrap(), hex_validators, validators.len() as i32)
                .expect("Unable to store data");
            Ok(())
        }
        // Err(_) => Err(diesel::result::Error::NotInTransaction),
        Err(_) => Err("❌ Failed to connect to WebSocket.".to_string()),
    }
}

fn delete_blockchain(id: i32) {
    let mut connection = establish_connection();

    match diesel::delete(schema::blockchain_info::table.find(id)).execute(&mut connection) {
        Ok(_) => println!("✅ Successfully deleted blockchain with ID {}.", id),
        Err(e) => println!("❌ Error deleting blockchain: {:?}", e),
    }
}
