use crate::{
    establish_connection, models::BlockDetails, models::Blockchain, rocket::cors::CORS,
    schema::block_details::dsl::*, schema::blockchain_info::dsl::*,
};
use diesel::RunQueryDsl;
use rocket::{
    get, routes,
    serde::{Deserialize, Serialize, json::Json},
};

#[derive(Serialize, Deserialize)]
pub struct Id {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Wss {
    endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataBlockchain {
    endpoint: String,
}

/// Returns all blockchain data stored in the database
#[get("/get_all_blockchains")]
pub fn get_all_blockchains() -> Json<Vec<Blockchain>> {
    let mut connection = establish_connection();

    let results = blockchain_info
        .load::<Blockchain>(&mut connection)
        .expect("Error loading blockchains");

    Json(results)
}

/// Returns all blockchain data stored in the database
#[get("/get_block_details")]
pub fn get_blocks_details() -> Json<Vec<BlockDetails>> {
    let mut connection = establish_connection();

    let results = block_details
        .load::<BlockDetails>(&mut connection)
        .expect("Error loading blockchains");

    Json(results)
}

/// Configure and mount the Rocket routes
pub fn rocket_routes() -> Vec<rocket::Route> {
    routes![get_all_blockchains, get_blocks_details]
}

// Rocket server launch configuration
pub async fn rocket_launch() {
    println!("🛰️ Launching the Rocket server... 🚀");
    let _ = rocket::build()
        .attach(CORS)
        .mount("/", crate::rocket_routes())
        .launch()
        .await;
}
