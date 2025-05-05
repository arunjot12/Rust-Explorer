use std::io::{self, Write};

pub fn main_menu() -> u32 {
    println!("📋 Choose:\n1️⃣ Start Rocket Server\n2️⃣ Store Blockchain Data");
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
