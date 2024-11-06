use reqwest::{
    header::{HeaderMap},
    Response,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

use crate::error::LfaError;

#[derive(Debug, Clone)]
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

    pub async fn get(&self, url: &str) -> Result<Response, LfaError> {
        let response = self
            .client
            .get(url)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(LfaError::from_response(response).await)
        }
    }

    pub async fn post(
        &self,
        url: &str,
        headers: HeaderMap,
        body: String,
    ) -> Result<Response, LfaError> {
        let response = self
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(LfaError::from_response(response).await)
        }
    }
}
