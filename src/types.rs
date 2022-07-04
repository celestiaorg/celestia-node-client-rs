use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    version: Version,
    chain_id: String,
    height: u32,
    time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    block: u32,
    app: u32,
}
