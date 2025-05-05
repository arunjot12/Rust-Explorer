use crate::establish_connection;
use crate::models::Blockchain;
use crate::schema::blockchain_info::dsl::*;
use diesel::RunQueryDsl;
use crate::rocket::cors::CORS; // if cors.rs is in the same crate
use rocket::serde::json::Json;
use rocket::{get, routes};

/// Returns all blockchain data stored in the database
#[get("/get_all_blockchains")]
pub fn get_all_blockchains() -> Json<Vec<Blockchain>> {
    let mut connection = establish_connection();

    let results = blockchain_info
        .load::<Blockchain>(&mut connection)
        .expect("Error loading blockchains");

    Json(results)
}

/// Configure and mount the Rocket routes
pub fn rocket_routes() -> Vec<rocket::Route> {
    routes![get_all_blockchains]
}

// Rocket server launch configuration
pub async fn rocket_launch() {
    println!("🛰️ Launching the Rocket server... 🚀");
    let _ = rocket::build()
        .attach(CORS)
        .mount("/get_all_blockchains", crate::rocket_routes())
        .launch()
        .await;
}
