// @generated automatically by Diesel CLI.

diesel::table! {
    block_details (block_number) {
        block_number -> Int4,
        parentshash -> Int4,
        extrinsic_count -> Int4,
        events -> Varchar,
    }
}

diesel::table! {
    blockchain_info (id) {
        id -> Int4,
        blockchain_name -> Varchar,
        validator_count -> Int4,
        validators -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(block_details, blockchain_info,);
