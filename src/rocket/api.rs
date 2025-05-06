use crate::delete_blockchain;
use crate::establish_ws_connection;
use crate::establish_connection;
use crate::models::Blockchain;
use crate::rocket::cors::CORS; // if cors.rs is in the same crate
use crate::schema::blockchain_info::dsl::*;
use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::serde::json::Value;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, routes};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
pub struct Id{
    id:i32
}

#[derive(Serialize, Deserialize)]
pub struct Wss{
    endpoint:String
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

// Verify Endpoint is working or not
#[post("/endpoint_checker", data = "<input>")]
pub async fn verify_wss(input: Json<Wss>) -> Result<Json<Value>, Status> {
    match establish_ws_connection(&input.endpoint).await {
        Ok(_) => {
            println!("✅ Connection Established! 🎉");
            Ok(Json(json!({ "status": "success", "message": "Connection Established!" })))
        },
        Err(error) => {
            println!("❌ {}", error);
            Err(Status::InternalServerError)
        }
    }
}

/// Returns all blockchain data stored in the database
#[post("/delete_blockchains", data = "<input>")]
pub fn api_delete_blockchain(input: Json<Id>) ->  &'static str {
    delete_blockchain(input.id);
    "Blockchain deleted successfully"
}

/// Configure and mount the Rocket routes
pub fn rocket_routes() -> Vec<rocket::Route> {
    routes![get_all_blockchains,api_delete_blockchain,verify_wss]
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

