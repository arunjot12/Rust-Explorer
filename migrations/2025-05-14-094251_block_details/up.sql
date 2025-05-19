-- Your SQL goes here

CREATE TABLE block_details ( 
    block_number SERIAL PRIMARY KEY,
    block_hash TEXT NOT NULL,
    parentshash TEXT NOT NULL,
    state_root TEXT NOT NULL,
    extrinsics_root TEXT NOT NULL,
    extrinsic_count INTEGER NOT NULL,
    events TEXT NOT NULL
)