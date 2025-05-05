use jsonrpsee::ws_client::WsClient;
use std::{io::{self,Error, Write}, vec};

mod blockchain_api;
mod blockchain_data;
pub use blockchain_data::*;
pub use blockchain_api::*;
pub use connection::*;

pub mod connection;
pub mod models;
pub mod schema;

#[tokio::main]
async fn main() {
    println!("🚀 Hello! Welcome to the Blockchain World 🌐\n");
    println!("📋 Choose an option:\n1️⃣  Start the Rocket Server\n2️⃣  Store Blockchain Data\n");

    let user_input = user_input().trim().parse::<u32>().unwrap_or(0);

    match user_input {
        1 => rocket_launch().await,
        2 => store_blockchain().await,
        _ => println!("❌ Invalid option. Please restart and choose 1 or 2."),
    }
}

/// Handle user input for the WebSocket endpoint
fn get_websocket_endpoint() -> String {
    print!("🔧 Please enter the WebSocket endpoint for the blockchain: ");
    io::stdout().flush().unwrap();

    let mut endpoint = String::new();
    io::stdin()
        .read_line(&mut endpoint)
        .expect("Failed to read line");

    endpoint.trim().to_string()
}

/// Handle user input for the selected option
fn get_selected_option() -> u32 {
    print!(
        "📋 Please choose what you want to see:\n1️⃣  Option 1: Blockchain Details\n2️⃣  Option 2: Nothing\n👉 Your choice: "
    );
    io::stdout().flush().unwrap();

    let mut option_input = String::new();
    io::stdin()
        .read_line(&mut option_input)
        .expect("Failed to read line");

    option_input.trim().parse().unwrap_or(0)
}

/// Fetch blockchain name from the WebSocket connection
async fn fetch_blockchain_name(client: WsClient) -> Result<String,Error> {
    let get_blockchain_name = get_blockchain_name(client).await;
    match get_blockchain_name {
        Ok(ref name) => println!("Blockchain name: {:?}", name),
        Err(ref err) => println!("Error retrieving blockchain name: {}", err),
    }
   get_blockchain_name
}

// Check the Data and store in the Blockchain
async fn store_blockchain(){
    println!("💾 Preparing to store blockchain data...");
    let endpoint = get_websocket_endpoint();

    // Check if endpoint starts with "ws"
    if endpoint.starts_with("ws") {
        println!("🔌 Connecting to the blockchain WebSocket endpoint...");

        match establish_ws_connection(&endpoint).await {
            Ok(client) => {
                println!("✅ Connection Established! 🎉");

                // Get user's choice and fetch blockchain name if option 1 is selected
                let selected_option = get_selected_option();
                if selected_option == 1 {
                   let name=  fetch_blockchain_name(client).await;
                   let validators =   current_validators().await;
                   let total_validators = validators.len() as i32;
                   println!("Total Active Validators = {:?}",total_validators);
                   let mut converted_validators :Vec<String>= vec![];
                   for i in &validators{
                    println!("List of validators {:?},",hex::encode(i.0));
                    let encoded_validator = hex::encode(i.0);
                    converted_validators.push(encoded_validator);
                   }
                    println!("Do you want to store in this in the database?");

                    let user_input = user_input();
                    let command: String = user_input.to_lowercase().trim().parse().unwrap();
                
                    match command.as_ref() {
                        "store" => store_db(&name.unwrap(),converted_validators,total_validators),
                        "exit" => println!("👋 Goodbye!"),
                        _ => println!("❗ Not a recognized keyword. Try again!"),
                    }

                } else {
                    println!("👋 Thank you for visiting the site. Have a great day!");
                }
            }
            Err(error) => println!("❌ {}", error),
        }
    } else {
        println!("⚠️ Invalid WebSocket endpoint. It should start with 'ws://' or 'wss://'.");
    }
}

// Take the input from the user
fn user_input() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command
}