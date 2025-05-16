-- Your SQL goes here

CREATE TABLE block_details ( 
    block_number SERIAL PRIMARY KEY,
    block_hash VARCHAR NOT NULL,
    parentshash VARCHAR NOT NULL,
    state_root VARCHAR NOt NULL,
    extrinsics_root VARCHAR NOT NULL,
    extrinsic_count INTEGER NOT NULL,
    events TEXT NOT NULL
)