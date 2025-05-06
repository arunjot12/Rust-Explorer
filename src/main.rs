mod blockchain;
pub mod cli;
pub mod rocket;
pub mod models;
pub mod schema;

use blockchain::{connection::*, data::*};
pub use cli::*;
use diesel::{QueryDsl, RunQueryDsl};
use models::Blockchain;
use rocket::api::*;

#[tokio::main]
async fn main() {
    println!("🚀 Hello! Welcome to the Blockchain World 🌐\n");

    match main_menu() {
        1 => rocket_launch().await,
        2 => store_blockchain().await,
        3 => verify_blockchain().await,
        _ => println!("❌ Invalid choice. Restart the program."),
    }
}

// Check the Data and store in the Blockchain
async fn store_blockchain() {
    println!("💾 Preparing to store blockchain data...");
    let endpoint = get_websocket_endpoint();

    // Check if endpoint starts with "ws"
    if !endpoint.starts_with("ws") {
        println!("⚠️ WebSocket endpoint must start with 'ws://' or 'wss://'.");
        return;
    }

    match establish_ws_connection(&endpoint).await {
        Ok(client) => {
            println!("✅ Connection Established! 🎉");

            // Get user's choice and fetch blockchain name if option 1 is selected
            print!(
                "📋 Please choose what you want to see:\n1️⃣  Option 1: Blockchain Details\n2️⃣  Option 2: Nothing\n👉 Your choice: "
            );
            if get_selected_option() != 1 {
                println!("👋 Thanks for visiting. Bye!");
                return;
            }

            let name = get_blockchain_name(client).await;
            let validators = current_validators(&endpoint).await;

            println!(
                "Blockchain: {:?}\nActive Validators: {}",
                name,
                validators.len()
            );

            let hex_validators: Vec<String> = validators
                .iter()
                .map(|v| format!("0x{}", hex::encode(v.0)))
                .inspect(|v| println!("Validator: {:?}", v))
                .collect();

            println!(
                "Store this in the database? Type '1' to store or any other key word to exit:"
            );

            if get_selected_option() != 1 {
                println!("👋 Goodbye!");
                return;
            }
            store_db(&name.unwrap(), hex_validators, validators.len() as i32)
        }
        Err(error) => println!("❌ {}", error),
    }
}

fn delete_blockchain(id:i32) {
    let mut connection = establish_connection();

        match diesel::delete(schema::blockchain_info::table.find(id))
            .execute(&mut connection)
        {
            Ok(_) => println!("✅ Successfully deleted blockchain with ID {}.", id),
            Err(e) => println!("❌ Error deleting blockchain: {:?}", e),
        }
} 


