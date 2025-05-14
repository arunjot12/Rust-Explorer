-- Your SQL goes here

CREATE TABLE block_details ( 
    block_number SERIAL PRIMARY KEY,
    parentshash VARCHAR NOT NULL,
    extrinsic_count INTEGER NOT NULL,
    events TEXT NOT NULL
)