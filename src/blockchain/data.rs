use jsonrpsee::{core::client::ClientT, ws_client::WsClient};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use serde::{Deserialize, Serialize};
use substrate_api_client::{
    Api, GetStorage,
    ac_primitives::DefaultRuntimeConfig,
    rpc::JsonrpseeClient,
};


#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    Hash,
    Encode,
    Debug,
    Decode,
    Serialize,
    Deserialize,
    MaxEncodedLen,
)]
pub struct AccountId20(pub [u8; 20]);

/// Fetch the current blockchain name
pub async fn get_blockchain_name(client: WsClient) -> Result<String, std::io::Error> {
    let chain_name: String = client
        .request("system_chain", jsonrpsee::core::params::ArrayParams::new())
        .await
        .expect("Failed to retrieve the chain name");

    Ok(chain_name)
}

/// Fetch the current blockchain name
pub async fn current_validators(endpoint: &str) -> Vec<AccountId20> {
    let client = JsonrpseeClient::new(endpoint).await.expect("REASON");
    let api = Api::<DefaultRuntimeConfig, _>::new(client).await.unwrap();
    let validators = api
        .get_storage::<Vec<AccountId20>>("Session", "Validators", None)
        .await
        .unwrap();
    validators.unwrap()
}
