use core::result::Result::Ok;
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc::RpcClient;
use subxt::{OnlineClient, PolkadotConfig};

#[derive(Debug)]
pub enum BlockStatus {
    Produced,
    Finalized,
}

pub async fn blockchain_explorer(
    endpoint: &str,
    _persist: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
        .await
        .expect("Api not Supported");

    let rpc_client = RpcClient::from_url(endpoint).await?;
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

    let mut blocks_sub = api.blocks().subscribe_all().await.expect("msg");

    while let Some(block) = blocks_sub.next().await {
        let block = block.expect("Not a number");

        let finalized_head = rpc.chain_get_finalized_head().await?;
        let finalized_header = rpc
            .chain_get_header(Some(finalized_head))
            .await?
            .ok_or("Finalized header not found")?;
        let keys: Vec<subxt::dynamic::Value> = vec![];

        let event_storage = subxt::dynamic::storage("System", "Number", keys);
        println!("Event storage:{:?}", event_storage);
        let value = api
            .storage()
            .at_latest()
            .await?
            .fetch(&event_storage)
            .await?
            .ok_or("No events found")?;

        let block_number = value.to_value()?; // This returns a serde_json::Value
        let converted_block_number = serde_json::to_string_pretty(&block_number)?;
        println!("Converted {}", converted_block_number);

        // Finalized block number
        let finalized_block_number = finalized_header.number;

        // Update earlier blocks finialized status
        //    let earlier_block_number = block.number() - 0;
        println!("Produced block number is {}", block.number());
        println!("Finalized block number is {}", finalized_block_number);
        // if finalized_block_number >= earlier_block_number {
        //     // println!("Finalized block number is {}",finalized_block_number);
        //     BlockStatus::Finalized
        // } else {
        //     BlockStatus::Produced
        // };
    }
    Ok(())
}
