use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::blockchain_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blockchain {
    pub id: i32,
    pub blockchain_name: String,
    pub validator_count: i32,
    pub validators: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::blockchain_info)]
pub struct NewBlockchain<'a> {
    pub blockchain_name: &'a str,
    pub validator_count: i32,
    pub validators: &'a str,
}

// block_number SERIAL PRIMARY KEY,
//     status TEXT NOT NULL,
//     era INTEGER NOT NULL,
//     block_size INTEGER NOT NULL,
//     gas_limit INTEGER NOT NULL,
//     gas_used INTEGER NOT NULL,
//     hash TEXT NOT NULL,
//     block_hash TEXT NOT NULL,
//     parentshash TEXT NOT NULL,
//     state_root TEXT NOT NULL,
//     total_transactions INTEGER NOT NULL,
//     withdrawal_transaction INTEGER NOT NULL,
//     contract_tx INTEGER NOT NULL


#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::blockchain_explorer)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockChainExplorer {
    pub block_number: i32,
    pub status: String,
    pub era: i32,
    pub block_size: i32,
    pub gas_limit: i32,
    pub gas_used: i32,
    pub hash: String,
    pub block_hash: String,
    pub parentshash: String,
    pub state_root: String,
    pub total_transactions: i32,
    pub withdrawal_transaction: i32,
    pub contract_tx: i32,
}