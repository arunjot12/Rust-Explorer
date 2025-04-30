use rocket::{get, routes, State};
use rocket::serde::json::Json;
use crate::models::Blockchain;
use crate::establish_connection;
use diesel::RunQueryDsl;
use crate::schema::blockchain_info::dsl::*;

/// Returns all blockchain data stored in the database
#[get("/blockchains")]
pub fn get_all_blockchains() -> Json<Vec<Blockchain>> {
    let mut connection = establish_connection();
    
    let results = blockchain_info
        .load::<Blockchain>(&mut connection)
        .expect("Error loading blockchains");
    
    Json(results)
}
