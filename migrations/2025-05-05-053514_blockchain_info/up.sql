-- Your SQL goes here

CREATE TABLE blockchain_info (
   id SERIAL PRIMARY KEY,
   blockchain_name VARCHAR NOT NULL,
   validator_count INTEGER NOT NULL,
   validators VARCHAR NOT NULL
);f