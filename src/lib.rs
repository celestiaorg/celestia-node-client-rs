//! Celestia Node REST API wrapper.
//!
//! Wraps the Celestia Node API: <https://docs.celestia.org/developers/node-api>.

pub use reqwest;
pub use reqwest::Error;
pub use serde::{Deserialize, Serialize};

pub const DEFAULT_BASE_URL: &str = "http://localhost:26658";

pub const ENDPOINT_BALANCE: &str = "/balance";

/// Celestia client context. Keeps track of a REST client against a base URL.
pub struct Context {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Deserialize, Serialize)]
pub struct BalanceResponse {
    denom: String,
    amount: u64,
}

impl Context {
    async fn call<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}/{}", &self.base_url, endpoint);
        let response = self.client.get(url).send().await?;
        let result = response.json::<T>().await?;
        Ok(result)
    }

    /// Initialize a new Celestia client context.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Call the `ENDPOINT_BALANCE` endpoint.
    ///
    /// Reference: <https://docs.celestia.org/developers/node-tutorial#balance>
    pub async fn balance(&self) -> Result<BalanceResponse, Error> {
        let response: BalanceResponse = self.call(ENDPOINT_BALANCE).await?;
        Ok(response)
    }
}
