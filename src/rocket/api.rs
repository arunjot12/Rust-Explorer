use crate::delete_blockchain;
use crate::establish_connection;
use crate::models::Blockchain;
use crate::rocket::cors::CORS; // if cors.rs is in the same crate
use crate::schema::blockchain_info::dsl::*;
use diesel::RunQueryDsl;
use crate::rocket::cors::options_delete_blockchain;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, routes};

#[derive(Serialize, Deserialize)]
pub struct Id{
    id:i32
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
#[post("/delete_blockchains", data = "<input>")]
pub fn api_delete_blockchain(input: Json<Id>) ->  &'static str {
    delete_blockchain(input.id);
    "Blockchain deleted successfully"
}

/// Configure and mount the Rocket routes
pub fn rocket_routes() -> Vec<rocket::Route> {
    routes![get_all_blockchains,api_delete_blockchain]
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

