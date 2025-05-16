use crate::{
    delete_blockchain, establish_connection, establish_ws_connection, models::Blockchain,models::BlockDetails,
    rocket::cors::CORS, schema::blockchain_info::dsl::*, store_blockchain,process_blocks, schema::block_details::dsl::*
};
use diesel::RunQueryDsl;
use rocket::{
    get,
    http::Status,
    post, routes,
    serde::{
        Deserialize, Serialize,
        json::{Json, Value},
    },
};
use serde_json::json;

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

// Verify Endpoint is working or not
#[post("/endpoint_checker", data = "<input>")]
pub async fn verify_wss(input: Json<Wss>) -> Result<Json<Value>, Status> {
    match establish_ws_connection(&input.endpoint).await {
        Ok(_) => {
            println!("✅ Connection Established! 🎉");
            Ok(Json(
                json!({ "status": "success", "message": "Connection Established!" }),
            ))
        }
        Err(error) => {
            println!("❌ {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// Verify Endpoint is working or not
// #[post("/store_block_details", data = "<input>")]
// pub async fn store_blocks_details(input: Json<DataBlockchain>) -> Result<Json<Value>, Status> {
//     match process_blocks(&input.endpoint,true).await {
//         Ok(_) => {
//             println!("✅ Storing Blocks Details! 🎉");
//             Ok(Json(
//                 json!({ "status": "success", "message": "Connection Established!" }),
//             ))
//         }
//         Err(error) => {
//             println!("❌ {}", error);
//             Err(Status::InternalServerError)
//         }
//     }
// }

/// Returns all blockchain data stored in the database
#[post("/delete_blockchains", data = "<input>")]
pub fn api_delete_blockchain(input: Json<Id>) -> &'static str {
    delete_blockchain(input.id);
    "Blockchain deleted successfully"
}

/// Configure and mount the Rocket routes
pub fn rocket_routes() -> Vec<rocket::Route> {
    routes![get_all_blockchains, api_delete_blockchain, verify_wss,store_blocks_details,get_blocks_details]
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
