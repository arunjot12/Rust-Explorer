use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use std::io;

#[tokio::main]
async fn main() {
    println!("Hello! Welcome to the Blockchain World");
    println!("Please enter the websocket data for the blockchain");

    let mut new = String::new();
    io::stdin()
        .read_line(&mut new)
        .expect("Failed to read line");

    let mut check: std::str::CharIndices<'_> = new.char_indices();
    if check.next() == Some((0, 'w')) && check.next() == Some((1, 's')) {
        println!("Checking the data");

        match get_blockchain_name(&new).await {
            Ok(name) => println!("Blockchain name is {:?}", name),
            Err(err) => println!("Error is {}", err),
        }
    } else {
        println!("Please enter the correct endpoint");
    }

    pub async fn get_blockchain_name(url: &str) -> Result<String, std::io::Error> {
        let client = WsClientBuilder::default().build(url).await.expect("REASON");
        let chain_name: String = client
            .request("system_chain", jsonrpsee::core::params::ArrayParams::new())
            .await
            .expect("Failed to retrieve the chain name");
        Ok(chain_name)
    }
}
