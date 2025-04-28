use std::io;
mod blockchain_data;
pub use blockchain_data::*;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};


#[tokio::main]
async fn main() {
    println!("Hello! Welcome to the Blockchain World");
    println!("Please enter the WebSocket endpoint for the blockchain:");

    let mut endpoint = String::new();
    io::stdin()
        .read_line(&mut endpoint)
        .expect("Failed to read line");

    let mut chars = endpoint.char_indices();

    if chars.next() == Some((0, 'w')) && chars.next() == Some((1, 's')) {
        println!("Checking the connection...");

        let client_result = WsClientBuilder::default().build(&endpoint.clone()).await;
        if client_result.is_ok() {
            println!("Connection Established!");
        } else {
            println!("Failed to connect. Please enter a valid WebSocket endpoint.");
            return;
        }

        println!("Please choose what you want to store:");
        println!("Option 1: Blockchain name");

        let mut option_input = String::new();
        io::stdin()
            .read_line(&mut option_input)
            .expect("Failed to read line");

        let selected_option: u32 = option_input.trim().parse().expect("Please type a number!");

        match selected_option {
            1 => fetch_blockchain_name(client_result.expect("msg")).await,
            _ => println!("Invalid option selected."),
        }
    } else {
        println!("Invalid WebSocket endpoint. It should start with 'ws'.");
    }
}

async fn fetch_blockchain_name(client: WsClient) {
    match get_blockchain_name(client).await {
        Ok(name) => println!("Blockchain name: {:?}", name),
        Err(err) => println!("Error retrieving blockchain name: {}", err),
    }
}
 
