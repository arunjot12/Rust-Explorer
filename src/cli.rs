use diesel::RunQueryDsl;
use std::io::{self, Write};

use crate::{blockchain::data::process_blocks, delete_blockchain, establish_connection, Blockchain};

pub fn main_menu() -> u32 {
    println!("📋 Choose:\n1️⃣ Start Rocket Server\n2️⃣ Show blockchain details on cli\n");
    prompt_number("👉 Your choice: ")
}

fn prompt_string(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_number(message: &str) -> u32 {
    prompt_string(message).parse().unwrap_or(0)
}

/// Handle user input for the WebSocket endpoint
pub fn get_websocket_endpoint() -> String {
    print!("🔧 Please enter the WebSocket endpoint for the blockchain: ");
    io::stdout().flush().unwrap();

    let mut endpoint = String::new();
    io::stdin()
        .read_line(&mut endpoint)
        .expect("Failed to read line");

    endpoint.trim().to_string()
}

/// Handle user input for the selected option
pub fn get_selected_option() -> u32 {
    io::stdout().flush().unwrap();
    let mut option_input = String::new();
    io::stdin()
        .read_line(&mut option_input)
        .expect("Failed to read line");

    option_input.trim().parse().unwrap_or(0)
}

pub async fn show_data_cli() {
    let endpoint = get_websocket_endpoint();
    if let Err(e) = process_blocks(&endpoint, false).await {
        eprintln!("❌ Error: {:?}", e);
    };
}

pub async fn store_blocks_details() {
    let endpoint = get_websocket_endpoint();
    if let Err(e) = process_blocks(&endpoint, true).await {
        eprintln!("❌ Error: {:?}", e);
    };
}

pub async fn verify_blockchain() {
    let mut connection = establish_connection();
    let results = crate::schema::blockchain_info::table
        .load::<Blockchain>(&mut connection)
        .expect("Some Error occured");

    println!("🌐 Current Blockchains:");

    let _: Vec<&Blockchain> = results
        .iter()
        .map(|v| v)
        .inspect(|v| println!("🆔  id {} ,📛 Name : {:?}", v.id, v.blockchain_name))
        .collect();

    println!("🗑️ Please enter the ID of the blockchain you want to delete:");

    let user_input = get_selected_option() as i32;
    let id: Vec<i32> = results.iter().map(|v| v.id).collect();

    if id.contains(&user_input) {
        delete_blockchain(user_input);
    } else {
        println!("⚠️ Invalid ID entered. No matching blockchain found.");
    }
}
