use scale_codec::{Decode, Encode, MaxEncodedLen};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use substrate_api_client::{
    Api, GetStorage, ac_primitives::DefaultRuntimeConfig, rpc::JsonrpseeClient,
};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::{OnlineClient, PolkadotConfig, backend::rpc::RpcClient};

// #[derive(Clone)]
#[subxt::subxt(runtime_metadata_path = "5irechain.scale")]
pub mod firechain {}

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
pub async fn get_blockchain_name(rpc: RpcClient) -> String {
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc.clone());
    rpc.system_name().await.expect("Not Valid")
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


pub async fn get_block_details(endpoint: &str) {
    let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
        .await
        .expect("Api not Supported");

    // Subscribe to new finalized blocks
    let mut blocks_sub = api.blocks().subscribe_finalized().await.expect("msg");

    println!("Listening to finalized blocks and printing events...\n");

    while let Some(block) = blocks_sub.next().await {
        let block = block.expect("msg");
        let block_number = block.number();
        println!("\n📦 Block #{block_number}");

        let extrinsics = block.extrinsics().await.unwrap();
        let transaction_length = extrinsics.len();


        let events = block.events().await.expect("2");

        for event in events.iter() {
            match event {
                Ok(ev) => {
                    let pallet = ev.pallet_name();
                    let variant = ev.variant_name();
                    println!("🎯 Event: {pallet}::{variant}");
                    println!("transaction_length first {:?}",transaction_length);

                    // Now try parsing the transfer event
                    if let Ok(Some(transfer)) =
                        ev.as_event::<firechain::balances::events::Transfer>()
                    {
                        println!(
                            "{:?} transfered {:?} to {:?} \n Transaction Length {:?}",
                            transfer.from.to_string(),
                            transfer.amount,
                            transfer.to.to_string(),
                            transaction_length
                        );
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to decode event: {e:?}");
                }
            }
        }
    }
}
