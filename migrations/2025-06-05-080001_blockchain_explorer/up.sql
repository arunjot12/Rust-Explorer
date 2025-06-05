-- Your SQL goes here
CREATE TABLE blockchain_explorer ( 
    block_number SERIAL PRIMARY KEY,
    status TEXT NOT NULL,
    era INTEGER NOT NULL,
    block_size INTEGER NOT NULL,
    gas_limit INTEGER NOT NULL,
    gas_used INTEGER NOT NULL,
    hash TEXT NOT NULL,
    block_hash TEXT NOT NULL,
    parentshash TEXT NOT NULL,
    state_root TEXT NOT NULL,
    total_transactions INTEGER NOT NULL,
    withdrawal_transaction INTEGER NOT NULL,
    contract_tx INTEGER NOT NULL
)