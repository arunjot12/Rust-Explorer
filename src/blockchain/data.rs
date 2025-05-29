use crate::establish_connection;
use crate::models::{BlockDetails, NewBlockDetails};
use core::result::Result::Ok;
use diesel::RunQueryDsl;
use jsonrpsee::{core::client::ClientT, ws_client::WsClient};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;
use substrate_api_client::{
    Api, GetStorage, ac_primitives::DefaultRuntimeConfig, rpc::JsonrpseeClient,
};
use subxt::config::Header;
use subxt::ext::subxt_core::alloc::string::ToString;
use subxt::{events, OnlineClient, PolkadotConfig};

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

pub async fn process_blocks(
    endpoint: &str,
    persist: bool,
) -> Result<(), Box<dyn std::error::Error>> {
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
        let block_hash = format!("0x{}", hex::encode(block.header().hash().as_ref()));
        let reward = events::EventsClient::new(api.clone()).at(block.header().hash());
        let extrinsics_root = format!("0x{}", hex::encode(block.header().extrinsics_root.as_ref()));
        let state_root = format!("0x{}", hex::encode(block.header().state_root));
        let keys: Vec<subxt::dynamic::Value> = vec![];

        let event_storage = subxt::dynamic::storage("System", "Events", keys);
        println!("Event storage:{:?}", event_storage);

        let value = api
            .storage()
            .at_latest()
            .await?
            .fetch(&event_storage)
            .await?
            .ok_or("No events found")?;
        // Convert dynamic::Value to serde_json::Value for easier debugging/printing
        let decoded_value = value.to_value()?; // This returns a `serde_json::Value`
        let a = serde_json::to_string_pretty(&decoded_value);
        println!("{}", serde_json::to_string_pretty(&decoded_value)?);
        // println!("Event value:{:?}",value);

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

                    let value; // fallback value

                    if let Ok(Some(transfer)) =
                        ev.as_event::<firechain::balances::events::Transfer>()
                    {
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
                        format!("{}::{}({})", pallet, variant, transfer_details);
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
            extrinsic_count: &(extrinsics.len() as i32),
            events: &all_events,
            block_hash: &block_hash.to_string(),
            state_root: &state_root,
            extrinsics_root: &extrinsics_root,
        };

        println!(
            "🧱 NewBlockDetails {{\n\
             block_number     : {},\n\
             parentshash      : {},\n\
             extrinsic_count  : {},\n\
             events           : {},\n\
             block_hash       : {},\n\
             state_root       : {},\n\
             extrinsics_root  : {}\n}}",
            new_details.block_number,
            new_details.parentshash,
            new_details.extrinsic_count,
            new_details.events,
            new_details.block_hash,
            new_details.state_root,
            new_details.extrinsics_root,
        );

        if persist {
            diesel::insert_into(crate::schema::block_details::table)
                .values(&new_details)
                .get_result::<BlockDetails>(&mut establish_connection())
                .expect("Failed");
        }
    }
    Ok(())
}
