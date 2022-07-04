//! Celestia Node REST API wrapper.
//!
//! Wraps the Celestia Node API: <https://docs.celestia.org/developers/node-api>.

mod types;

pub use reqwest;
pub use reqwest::Error;
pub use serde::{Deserialize, Serialize};
pub use types::*;

pub const DEFAULT_BASE_URL: &str = "http://localhost:26658";

const ENDPOINT_BALANCE: &str = "balance";
const ENDPOINT_HEADER: &str = "header";
const ENDPOINT_NAMESPACED_DATA: &str = "namespaced_data";
const ENDPOINT_NAMESPACED_SHARES: &str = "namespaced_shares";

const KEY_HEIGHT: &str = "height";

/// Celestia client context. Keeps track of a REST client against a base URL.
pub struct Context {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceResponse {
    denom: String,
    amount: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderResponse {
    header: Header,
    commit: Commit,
    validator_set: ValidatorSet,
    dah: DataAvailabilityHeader,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamespacedDataResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamespacedSharesResponse {}

impl Context {
    async fn call<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: String,
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
        let response = self.call(ENDPOINT_BALANCE.to_string()).await?;
        Ok(response)
    }

    /// Call the `ENDPOINT_HEADER` endpoint.
    ///
    /// Reference: <https://docs.celestia.org/developers/node-tutorial#get-block-header>
    ///
    /// # Arguments
    ///
    /// * `height` - Block height to fetch header for. Must be > 0.
    pub async fn header(&self, height: u64) -> Result<HeaderResponse, Error> {
        let response = self.call(format!("{}/{}", ENDPOINT_HEADER, height)).await?;
        Ok(response)
    }

    /// Call the `ENDPOINT_NAMESPACED_DATA` endpoint.
    ///
    /// # Arguments
    ///
    /// * `height` - Block height to fetch data for. Must be > 0.
    pub async fn namespaced_data(
        &self,
        height: Option<u64>,
    ) -> Result<NamespacedDataResponse, Error> {
        let response = match height {
            Some(height) => {
                self.call(format!(
                    "{}/{}/{}",
                    ENDPOINT_NAMESPACED_DATA, KEY_HEIGHT, height
                ))
                .await?
            }
            None => self.call(ENDPOINT_NAMESPACED_DATA.to_string()).await?,
        };
        Ok(response)
    }
}

/// Tests for client. Note that you need to be running a Celestia node at
/// `DEFAULT_BASE_URL`.
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn balance() {
        let context = Context::new(DEFAULT_BASE_URL);
        let balance_response = context.balance().await.unwrap();
        println!("{} response: {:?}", ENDPOINT_BALANCE, balance_response);
    }

    #[tokio::test]
    async fn header() {
        let context = Context::new(DEFAULT_BASE_URL);
        let header_response = context.header(1).await.unwrap();
        println!("{} response: {:?}", ENDPOINT_HEADER, header_response);
    }
}
