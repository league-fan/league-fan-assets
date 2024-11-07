use anyhow::Result;
use log::info;
use reqwest::Response;
use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    download_url: Option<String>,
    img_id: String,
    force: bool,
}

impl ClientTrait for R2Client {
    fn default() -> Self {
        Self::try_from_env().unwrap()
    }

    async fn do_task(&self, task: &AssetsTask) -> Result<(), LfaError> {
        let url = task.url.clone();
        let name = task.path.trim_start_matches('/').to_string();

        let resp = match self.upload_file(&url, &name).await {
            Ok(resp) => resp,
            Err(LfaError::FileExists(_)) => return Ok(()),
            Err(LfaError::FileNotExists(_)) => {
                if task.fallback_url.is_none() {
                    return Err(LfaError::UnexpectedStatusCode(400, url));
                }
                info!("Url resource not found, try fallback: {}", url);
                let fallback_url = task.fallback_url.as_ref().unwrap();
                match self.upload_file(fallback_url, &name).await {
                    Ok(resp) => resp,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        };
        match resp.status().as_u16() {
            200 => Ok(()),
            _ => Err(LfaError::UploadFailed(resp.status().as_u16(), resp.text().await?)),
        }
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

    pub async fn upload_file(&self, download_url: &str, img_id: &str) -> Result<Response, LfaError> {
        let endpoint = format!("{}/fetchSave", self.worker_url);

        let mut heades = reqwest::header::HeaderMap::new();
        heades.insert("Content-Type", "application/json".parse().unwrap());
        heades.insert(
            "Authorization",
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        let body = serde_json::to_string(&UploadRequest {
            download_url: Some(download_url.to_string()),
            img_id: img_id.to_string(),
            force: false,
        })?;

        self.client.post(&endpoint, heades, body).await
    }

    pub async fn delete_file(&self, img_id: &str) -> Result<Response, LfaError> {
        let endpoint = format!("{}/delete", self.worker_url);

        let mut heades = reqwest::header::HeaderMap::new();
        heades.insert("Content-Type", "application/json".parse().unwrap());
        heades.insert(
            "Authorization",
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        let body = serde_json::to_string(&UploadRequest {
            download_url: None,
            img_id: img_id.to_string(),
            force: false,
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
        assert!(matches!(result, Err(LfaError::FileNotExists(_))));
    }

    // #[tokio::test]
    // async fn test_upload_file_already_exist() {
    //     let client = R2Client::try_from_env().unwrap();
    //     let download_url = VALID_URL;
    //     let name = "test_exist.png";
    //     let result = client.upload_file(download_url, name).await;
    //     assert!(result.is_err());
    //     assert!(matches!(result, Err(LfaError::FileExists(_))));
    // }

    #[tokio::test]
    async fn test_upload_file_and_delete() {
        let client = R2Client::try_from_env().unwrap();
        let download_url = VALID_URL;
        let name = "test_success.png";
        let result = client.upload_file(download_url, name).await;
        assert!(result.is_ok());

        // let result = client.delete_file(name).await;
        // assert!(result.is_ok());
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
