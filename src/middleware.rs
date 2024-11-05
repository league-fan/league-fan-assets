use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::de::DeserializeOwned;

use crate::types::utils::{get_cdragon_url, Config, FALLBACK_CONFIG};

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

    pub async fn assets_get(&self, ori_url: &str, config: &Arc<Config>) -> Result<Response> {
        let url = get_cdragon_url(ori_url, config);
        let response = self.client.get(&url).send().await;

        if let Ok(response) = response {
            if response.status().is_success() {
                debug!("Get OK, url: {}", url);
                return Ok(response);
            } else {
                debug!(
                    "Get failed with status: {}, try fallback {}",
                    response.status(),
                    url
                );
            }
        } else {
            debug!(
                "Get failed: {}, try fallback {}",
                response.unwrap_err(),
                url
            );
        }

        let fallback_url = get_cdragon_url(ori_url, &FALLBACK_CONFIG);
        let fallback_response = self.client.get(&fallback_url).send().await?;

        if fallback_response.status().is_success() {
            debug!("Get OK, url: {}", fallback_url);
            Ok(fallback_response)
        } else {
            Err(anyhow!(
                "Get fallback failed with status: {}",
                fallback_response.status()
            ))
        }
    }

    pub async fn get_bytes(&self, ori_url: &str, config: &Arc<Config>) -> Result<Vec<u8>> {
        Ok(self
            .assets_get(ori_url, config)
            .await?
            .bytes()
            .await?
            .to_vec())
    }

    pub async fn get_text(&self, url: &str) -> Result<String> {
        Ok(self.get(url).await?.text().await?)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        Ok(self.get(url).await?.json().await?)
    }
}
