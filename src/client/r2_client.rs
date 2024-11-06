use anyhow::Result;
use log::info;
use reqwest::Response;
use serde::Serialize;

use crate::error::LfaError;

use super::{
    client_trait::{AssetsTask, ClientTrait},
    fetch_client::FetchClient,
};

#[derive(Debug, Clone)]
pub struct R2Client {
    client: FetchClient,
    worker_url: String,
    token: String,
}

#[derive(Serialize)]
pub struct UploadRequest {
    url: Option<String>,
    name: String,
}

impl ClientTrait for R2Client {
    fn default() -> Self {
        Self::try_from_env().unwrap()
    }

    async fn do_task(&self, task: &AssetsTask) -> Result<(), crate::error::LfaError> {
        let url = task.url.clone();
        let name = task.path.trim_start_matches('/').to_string();

        match self.upload_file(&url, &name).await {
            Ok(resp) => resp,
            Err(LfaError::FileExists(_)) => {
                return Ok(());
            }
            Err(LfaError::InternalServerError(_)) => {
                if task.fallback_url.is_none() {
                    return Err(LfaError::FileNotExists(url));
                }
                info!("Internal server error, try fallback: {}", url);
                let fallback_url = task.fallback_url.as_ref().unwrap();
                self.upload_file(fallback_url, &name).await?
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

impl R2Client {
    pub fn try_from_env() -> Result<Self> {
        let worker_url = std::env::var("R2_WORKER_URL")?;
        let token = std::env::var("R2_TOKEN")?;
        let client = FetchClient::default();
        Ok(R2Client {
            client,
            worker_url,
            token,
        })
    }

    pub async fn upload_file(&self, url: &str, name: &str) -> Result<Response, LfaError> {
        let endpoint = format!("{}/fetchSave", self.worker_url);

        let mut heades = reqwest::header::HeaderMap::new();
        heades.insert("Content-Type", "application/json".parse().unwrap());
        heades.insert(
            "Authorization",
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        let body = serde_json::to_string(&UploadRequest {
            url: Some(url.to_string()),
            name: name.to_string(),
        })?;

        self.client.post(&endpoint, heades, body).await
    }

    pub async fn delete_file(&self, name: &str) -> Result<Response, LfaError> {
        let endpoint = format!("{}/delete", self.worker_url);

        let mut heades = reqwest::header::HeaderMap::new();
        heades.insert("Content-Type", "application/json".parse().unwrap());
        heades.insert(
            "Authorization",
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        let body = serde_json::to_string(&UploadRequest {
            url: None,
            name: name.to_string(),
        })?;

        self.client.post(&endpoint, heades, body).await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const VALID_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/assets/loot/loottable_chest_generic_1.png";
    const INVALID_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/assets/loot/loottable_chest_generic_1.png";

    #[tokio::test]
    async fn test_upload_file_fail_to_download() {
        let client = R2Client::try_from_env().unwrap();
        let download_url = INVALID_URL;
        let name = "test_unreachable.png";
        let result = client.upload_file(download_url, name).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(LfaError::InternalServerError(_))));
    }

    #[tokio::test]
    async fn test_upload_file_already_exist() {
        let client = R2Client::try_from_env().unwrap();
        let download_url = VALID_URL;
        let name = "test_exist.png";
        let result = client.upload_file(download_url, name).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(LfaError::FileExists(_))));
    }

    #[tokio::test]
    async fn test_upload_file_and_delete() {
        let client = R2Client::try_from_env().unwrap();
        let download_url = VALID_URL;
        let name = "test_success.png";
        let result = client.upload_file(download_url, name).await;
        assert!(result.is_ok());

        let result = client.delete_file(name).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_file_not_exist() {
        let client = R2Client::try_from_env().unwrap();
        let name = "test_not_exist.png";
        let result = client.delete_file(name).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(LfaError::FileNotExists(_))));
    }
}
