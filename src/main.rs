use std::io::{self, Write};
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use diesel::prelude::*;
use dotenvy::dotenv;
use models::NewBlockchain;
use self::models::Blockchain;
use std::env;

mod blockchain_api;
mod blockchain_data;
pub use blockchain_data::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Establish WebSocket connection and handle errors
async fn establish_ws_connection(endpoint: &str) -> Result<WsClient, String> {
    WsClientBuilder::default()
        .build(endpoint)
        .await
        .map_err(|_| "Failed to connect. Please enter a valid WebSocket endpoint.".to_string())
}

/// Handle user input for the WebSocket endpoint
fn get_websocket_endpoint() -> String {
    print!("Please enter the WebSocket endpoint for the blockchain: ");
    io::stdout().flush().unwrap();
    
    let mut endpoint = String::new();
    io::stdin()
        .read_line(&mut endpoint)
        .expect("Failed to read line");
    
    endpoint.trim().to_string()
}

// Store the name in the database
fn store_blockchain(blockchain:&str){

    let new_blockchain = NewBlockchain { blockchain_name:blockchain };

    diesel::insert_into(schema::blockchain_info::table)
    .values(&new_blockchain)
    .get_result::<Blockchain>(&mut establish_connection())
    .expect("Error saving new post");
}

/// Handle user input for the selected option
fn get_selected_option() -> u32 {
    print!("Please choose what you want to store:\nOption 1: Blockchain name\n> ");
    io::stdout().flush().unwrap();

    let mut option_input = String::new();
    io::stdin()
        .read_line(&mut option_input)
        .expect("Failed to read line");

    option_input.trim().parse().unwrap_or(0)
}

/// Fetch blockchain name from the WebSocket connection
async fn fetch_blockchain_name(client: WsClient){
    let get_blockchain_name = get_blockchain_name(client).await;
    match get_blockchain_name {
        Ok(ref name) => println!("Blockchain name: {:?}", name),
        Err(ref err) => println!("Error retrieving blockchain name: {}", err),
    }


    println!("Do you want to store in the database? ");

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command: String = command.to_lowercase().trim().parse().unwrap();

    let blockchain = get_blockchain_name.unwrap();

    match command.as_ref() {
        "store" => store_blockchain(&blockchain),
        "exit" =>  println!("Thank you "),
        _ => println!("Not a correct key word")
    }
}

#[tokio::main]
async fn main() {
    println!("Hello! Welcome to the Blockchain World");

    let endpoint = get_websocket_endpoint();
 
    // Check if endpoint starts with "ws"
    if endpoint.starts_with("ws") {
        println!("Checking the connection...");
        
        match establish_ws_connection(&endpoint).await {
            Ok(client) => {
                println!("Connection Established!");
                
                // Get user's choice and fetch blockchain name if option 1 is selected
                let selected_option = get_selected_option();
                if selected_option == 1 {
                    fetch_blockchain_name(client).await;
                } else {
                    println!("Invalid option selected.");
                }


            }
            Err(error) => println!("{}", error),
        }
    } else {
        println!("Invalid WebSocket endpoint. It should start with 'ws'.");
    }
}
