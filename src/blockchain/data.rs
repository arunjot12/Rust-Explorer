use crate::establish_connection;
use crate::models::{BlockDetails, NewBlockDetails};
use crate::schema::block_details::parentshash;
use diesel::RunQueryDsl;
use jsonrpsee::{core::client::ClientT, ws_client::WsClient};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use serde::{Deserialize, Serialize};
use subxt::config::Header;
use subxt::ext::subxt_core::alloc::string::ToString;
use std::fmt;
use std::fmt::Debug;
use substrate_api_client::{
    Api, GetStorage, ac_primitives::DefaultRuntimeConfig, rpc::JsonrpseeClient,
};
use subxt::{OnlineClient, PolkadotConfig};

// #[derive(Clone)]
#[subxt::subxt(runtime_metadata_path = "5irechain.scale")]
pub mod firechain {}

#[derive(Debug, Clone)]
struct BalanceTransfer {
    transfer_from: String,
    transfer_dest: String,
    amount: u128,
}

impl fmt::Display for BalanceTransfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "From: {}, To: {}, Amount: {}",
            self.transfer_from, self.transfer_dest, self.amount
        )
    }
}
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
                    println!("transaction_length first {:?}", transaction_length);

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

pub async fn store_block_details(endpoint: &str) {
    let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
        .await
        .expect("Api not Supported");

    // Subscribe to new finalized blocks
    let mut blocks_sub = api.blocks().subscribe_finalized().await.expect("msg");

    println!("Listening to finalized blocks and printing events...\n");

    while let Some(block) = blocks_sub.next().await {
        let block = block.expect("msg");
        let block_number = block.number() as i32;
        let parent_hash = block.header().parent_hash.to_string();
        let block_hash = block.header().hash().to_string();
        let exitrinsic = block.header().extrinsics_root.to_string();
        let state_root = block.header().state_root.to_string();
        // println!("\n📦 Block #{block_number}");
        // println!("\n📦 Block Header #{:?}", block_hash);

        let extrinsics = block.extrinsics().await.unwrap();

        let events = block.events().await.expect("no events ");

        let mut all_events = String::new();

        for event in events.iter() {
            match event {
                Ok(ev) => {
                    let pallet = ev.pallet_name();
                    let variant = ev.variant_name();
                     println!("🎯 Event: {pallet}::{variant}");
                     println!("transaction_length first {:?}", extrinsics.len());
                     
                    let mut value = String::new(); // fallback value

                    if let Ok(Some(transfer)) = ev.as_event::<firechain::balances::events::Transfer>() {
                        let transfer_details = BalanceTransfer {
                            transfer_from: transfer.from.to_string(),
                            transfer_dest: transfer.to.to_string(),
                            amount: transfer.amount,
                        };
                    
                        value = pallet.to_owned() + variant + &transfer_details.to_string();
                    
                        println!(
                            "{:?} transferred {:?} to {:?} \nTransaction Length {:?}",
                            transfer.from.to_string(),
                            transfer.amount,
                            transfer.to.to_string(),
                            extrinsics.len()
                        );
                    } else {
                        value = pallet.to_owned() + variant + "";
                    }
                    all_events.push_str(&value);
                    all_events.push_str(" | ")
                    
                }

                Err(e) => {
                    println!("⚠️ Failed to decode event: {e:?}");
                }
            }

        }
        let new_details = NewBlockDetails {
            block_number: &block_number,
            parentshash: &parent_hash,
            extrinsic_count: extrinsics.len() as i32,
            events: &all_events,
        };
        
        diesel::insert_into(crate::schema::block_details::table)
            .values(&new_details)
            .get_result::<BlockDetails>(&mut establish_connection())
            .expect("Failed");
        

    }
}
