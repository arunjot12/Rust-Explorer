
use jsonrpsee::{core::client::ClientT, ws_client::WsClient};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient,PolkadotConfig};
use std::fmt::Debug;
use substrate_api_client::{
    ac_primitives::{DefaultRuntimeConfig, H256}, rpc::JsonrpseeClient, Api, GetStorage
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

/// Fetch the current blockchain name
pub async fn get_current_block(endpoint: &str) -> u32 {
    let client = JsonrpseeClient::new(endpoint).await.expect("REASON");
    let api = Api::<DefaultRuntimeConfig, _>::new(client).await.unwrap();
    let current_block_number = api
        .get_storage::<u32>("System", "Number", None)
        .await
        .unwrap();
    current_block_number.unwrap()
}

pub async fn get_block_event(endpoint: &str) {
    let client = JsonrpseeClient::new(endpoint).await.expect("Connection failed");
    let api = Api::<DefaultRuntimeConfig, _>::new(client).await.unwrap();

    let block_number = api
        .get_storage::<u32>("System", "Number", None)
        .await
        .unwrap()
        .unwrap();

    let block_hash :Option<H256>= api
        .get_storage_map("System", "BlockHash", block_number -1 , None)
        .await
        .unwrap();

     println!("*****block hash is {:?}, block number is {:?}",block_hash,block_number);

    let api = OnlineClient::<PolkadotConfig>::from_url(endpoint).await;

     // Subscribe to new finalized blocks
     let mut blocks_sub = api.expect("REASON").blocks().subscribe_finalized().await.expect("msg");

     println!("Listening to finalized blocks and printing events...\n");

     while let Some(block) = blocks_sub.next().await {
         let block = block.expect("msg");
         let block_number = block.number();
         println!("\n📦 Block #{block_number}");
 
         let events = block.events().await.expect("2");
 
         for event in events.iter() {
             match event {
                 Ok(ev) => {
                     let pallet = ev.pallet_name();
                     let variant = ev.variant_name();
                     println!("🎯 Event: {pallet}::{variant}");
                 },
                 Err(e) => {
                     println!("⚠️ Failed to decode event: {e:?}");
                 }
             }
         }
     }

}