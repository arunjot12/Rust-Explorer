-- Your SQL goes here
CREATE TABLE block_details ( 
    block_number SERIAL PRIMARY KEY,
    parentshash INTEGER NOT NULL,
    extrinsic_count INTEGER NOT NULL,
    events VARCHAR NOT NULL
)