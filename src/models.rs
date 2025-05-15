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

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::block_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockDetails {
    pub block_number: i32,
    pub parentshash: String,
    pub extrinsic_count: i32,
    pub events: String,
}

#[derive(Insertable,Debug)]
#[diesel(table_name = crate::schema::block_details)]
pub struct NewBlockDetails<'a> {
    pub block_number: &'a i32,
    pub parentshash: &'a str,
    pub extrinsic_count: i32,
    pub events: &'a str,
}
