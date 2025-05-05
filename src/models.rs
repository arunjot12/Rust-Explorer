use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::blockchain_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blockchain {
    pub id: i32,
    pub blockchain_name: String,
    pub validator_count:i32,
    pub validators: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::blockchain_info)]
pub struct NewBlockchain<'a> {
    pub blockchain_name: &'a str,
    pub validator_count:i32,
    pub validators: &'a str
}
