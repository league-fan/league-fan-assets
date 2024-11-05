use anyhow::Result;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::de::DeserializeOwned;

pub struct FetchClient {
    pub client: ClientWithMiddleware,
}

impl FetchClient {
    pub fn new(max_retries: u32) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self { client }
    }

    pub fn default() -> Self {
        Self::new(3)
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        self.client.get(url).send().await.map_err(|e| e.into())
    }

    pub async fn get_bytes(&self, url: &str) -> Result<Vec<u8>> {
        self.get(url)
            .await?
            .bytes()
            .await
            .map(|bytes| bytes.to_vec())
            .map_err(|e| e.into())
    }

    pub async fn get_text(&self, url: &str) -> Result<String> {
        self.get(url).await?.text().await.map_err(|e| e.into())
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.get(url).await?.json().await.map_err(|e| e.into())
    }
}
