// @generated automatically by Diesel CLI.

diesel::table! {
    block_details (block_number) {
        block_number -> Int4,
        block_hash -> Text,
        parentshash -> Text,
        state_root -> Text,
        extrinsics_root -> Text,
        extrinsic_count -> Int4,
        events -> Text,
    }
}

diesel::table! {
    blockchain_explorer (block_number) {
        block_number -> Int4,
        status -> Text,
        era -> Int4,
        block_size -> Int4,
        gas_limit -> Int4,
        gas_used -> Int4,
        hash -> Text,
        block_hash -> Text,
        parentshash -> Text,
        state_root -> Text,
        total_transactions -> Int4,
        withdrawal_transaction -> Int4,
        contract_tx -> Int4,
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

diesel::allow_tables_to_appear_in_same_query!(
    block_details,
    blockchain_explorer,
    blockchain_info,
);
