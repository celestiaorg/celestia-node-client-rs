//! Celestia Node REST API wrapper.
//!
//! Wraps the Celestia Node API: <https://docs.celestia.org/developers/node-api>.

pub use reqwest;
pub use serde::{Deserialize, Serialize};

pub const DEFAULT_BASE_URL: &str = "http://localhost:26658";
