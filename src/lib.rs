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
const ENDPOINT_DATA_AVAILABLE: &str = "data_available";
const ENDPOINT_HEAD: &str = "head";
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
    amount: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataAvailableResponse {
    available: bool,
    probability_of_availability: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeadResponse {
    header: Header,
    commit: Commit,
    validator_set: ValidatorSet,
    dah: DataAvailabilityHeader,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderResponse {
    header: Header,
    commit: Commit,
    validator_set: ValidatorSet,
    dah: DataAvailabilityHeader,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamespacedDataResponse {
    data: Vec<String>,
    height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamespacedSharesResponse {
    shares: Option<Vec<String>>,
    height: u32,
}

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
    ///
    /// # Arguments
    ///
    /// * `address` - Address to fetch for. If `None`, fetch from default address.
    pub async fn balance(&self, address: Option<String>) -> Result<BalanceResponse, Error> {
        let url = match address {
            Some(address) => {
                format!("{}/{}", ENDPOINT_BALANCE, address)
            }
            None => ENDPOINT_BALANCE.to_string(),
        };
        let response = self.call(url).await?;
        Ok(response)
    }

    /// Call the `ENDPOINT_DATA_AVAILABLE` endpoint.
    ///
    /// Reference: <https://docs.celestia.org/developers/node-tutorial#get-block-header>
    ///
    /// # Arguments
    ///
    /// * `height` - Block height to check availability for. Must be > 0.
    pub async fn data_available(&self, height: u64) -> Result<DataAvailableResponse, Error> {
        let response = self
            .call(format!("{}/{}", ENDPOINT_DATA_AVAILABLE, height))
            .await?;
        Ok(response)
    }

    /// Call the `ENDPOINT_HEAD` endpoint.
    pub async fn head(&self) -> Result<HeadResponse, Error> {
        let response = self.call(ENDPOINT_HEAD.to_string()).await?;
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
    /// * `namespace_id` - Namespace ID of the data to fetch.
    /// * `height` - Block height from which to fetch. If `None`, fetch from
    /// latest block.
    pub async fn namespaced_data(
        &self,
        namespace_id: String,
        height: Option<u64>,
    ) -> Result<NamespacedDataResponse, Error> {
        let url = match height {
            Some(height) => {
                format!(
                    "{}/{}/{}/{}",
                    ENDPOINT_NAMESPACED_DATA, namespace_id, KEY_HEIGHT, height
                )
            }
            None => {
                format!("{}/{}", ENDPOINT_NAMESPACED_DATA, namespace_id)
            }
        };
        let response = self.call(url).await?;
        Ok(response)
    }

    /// Call the `ENDPOINT_NAMESPACED_SHARES` endpoint.
    ///
    /// # Arguments
    ///
    /// * `namespace_id` - Namespace ID of the data to fetch.
    /// * `height` - Block height from which to fetch. If `None`, fetch from
    /// latest block.
    pub async fn namespaced_shares(
        &self,
        namespace_id: String,
        height: Option<u64>,
    ) -> Result<NamespacedSharesResponse, Error> {
        let url = match height {
            Some(height) => {
                format!(
                    "{}/{}/{}/{}",
                    ENDPOINT_NAMESPACED_SHARES, namespace_id, KEY_HEIGHT, height
                )
            }
            None => {
                format!("{}/{}", ENDPOINT_NAMESPACED_SHARES, namespace_id)
            }
        };
        let response = self.call(url).await?;
        Ok(response)
    }
}

/// Tests for client. Note that you need to be running a Celestia node at
/// `DEFAULT_BASE_URL`.
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn balance_none() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context.balance(None).await.unwrap();
        println!("{} response: {:?}", ENDPOINT_BALANCE, response);
    }

    #[tokio::test]
    async fn balance_some() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context.balance(Some("0".to_string())).await.unwrap();
        println!("{} response: {:?}", ENDPOINT_BALANCE, response);
    }

    #[tokio::test]
    async fn data_available() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context.data_available(1).await.unwrap();
        println!("{} response: {:?}", ENDPOINT_DATA_AVAILABLE, response);
    }

    #[tokio::test]
    async fn head() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context.head().await.unwrap();
        println!("{} response: {:?}", ENDPOINT_HEAD, response);
    }

    #[tokio::test]
    async fn header() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context.header(1).await.unwrap();
        println!("{} response: {:?}", ENDPOINT_HEADER, response);
    }

    #[tokio::test]
    async fn namespaced_data_none() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context
            .namespaced_data("0123456789abcdef".to_string(), None)
            .await
            .unwrap();
        println!("{} response: {:?}", ENDPOINT_NAMESPACED_DATA, response);
    }

    #[tokio::test]
    async fn namespaced_data_some() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context
            .namespaced_data("0123456789abcdef".to_string(), Some(1))
            .await
            .unwrap();
        println!("{} response: {:?}", ENDPOINT_NAMESPACED_DATA, response);
    }

    #[tokio::test]
    async fn namespaced_shares_none() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context
            .namespaced_shares("0123456789abcdef".to_string(), None)
            .await
            .unwrap();
        println!("{} response: {:?}", ENDPOINT_NAMESPACED_SHARES, response);
    }

    #[tokio::test]
    async fn namespaced_shares_some() {
        let context = Context::new(DEFAULT_BASE_URL);
        let response = context
            .namespaced_shares("0123456789abcdef".to_string(), Some(1))
            .await
            .unwrap();
        println!("{} response: {:?}", ENDPOINT_NAMESPACED_SHARES, response);
    }
}
