use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClient;

pub async fn get_blockchain_name(client: WsClient) -> Result<String, std::io::Error> {
    let chain_name: String = client
        .request("system_chain", jsonrpsee::core::params::ArrayParams::new())
        .await
        .expect("Failed to retrieve the chain name");

    Ok(chain_name)
}
