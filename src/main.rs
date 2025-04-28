use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use std::io;

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

        let client_result = WsClientBuilder::default().build(endpoint.clone()).await;
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
            1 => fetch_blockchain_name(&endpoint).await,
            _ => println!("Invalid option selected."),
        }
    } else {
        println!("Invalid WebSocket endpoint. It should start with 'ws'.");
    }
}

async fn fetch_blockchain_name(endpoint: &str) {
    match get_blockchain_name(endpoint).await {
        Ok(name) => println!("Blockchain name: {:?}", name),
        Err(err) => println!("Error retrieving blockchain name: {}", err),
    }
}

async fn get_blockchain_name(endpoint: &str) -> Result<String, std::io::Error> {
    let client = WsClientBuilder::default()
        .build(endpoint)
        .await
        .expect("Failed to build WebSocket client");

    let chain_name: String = client
        .request("system_chain", jsonrpsee::core::params::ArrayParams::new())
        .await
        .expect("Failed to retrieve the chain name");

    Ok(chain_name)
}
