use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use jsonrpsee_ws_client::Serialize;

#[derive(Queryable,Serialize, Selectable)]
#[diesel(table_name = crate::schema::blockchain_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blockchain {
    pub id: i32,
    pub blockchain_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::blockchain_info)]
pub struct NewBlockchain<'a> {
    pub blockchain_name: &'a str,
}
