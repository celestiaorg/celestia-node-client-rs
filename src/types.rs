use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    version: Version,
    chain_id: String,
    height: u32,
    time: DateTime<Utc>,
    last_block_id: BlockId,
    last_commit_hash: String,
    data_hash: String,
    validators_hash: String,
    next_validators_hash: String,
    consensus_hash: String,
    app_hash: String,
    last_results_hash: String,
    evidence_hash: String,
    proposer_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    block: u32,
    app: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockId {
    hash: String,
    parts: Parts,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parts {
    total: u32,
    hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    height: u32,
    round: u32,
    block_id: BlockId,
    signatures: Vec<Signature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    block_id_flag: u32,
    validator_address: String,
    timestamp: DateTime<Utc>,
    signature: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidatorSet {
    validators: Vec<Validator>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Validator {
    address: String,
    pub_key: String,
    voting_power: u32,
    proposer_priority: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataAvailabilityHeader {
    row_roots: Vec<String>,
    column_roots: Vec<String>,
}
