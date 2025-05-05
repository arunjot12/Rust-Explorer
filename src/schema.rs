// @generated automatically by Diesel CLI.

diesel::table! {
    blockchain_info (id) {
        id -> Int4,
        blockchain_name -> Varchar,
        validator_count -> Int4,
        validators -> Varchar,
    }
}
