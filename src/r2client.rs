use anyhow::{anyhow, Result};
use log::{info, warn};
use reqwest::Client;
use serde::Serialize;

pub struct R2Client {
    client: Client,
    worker_url: String,
    token: String,
}

#[derive(Serialize)]
struct UploadRequest {
    url: String,
    name: String,
}

impl R2Client {
    pub fn try_from_env() -> Result<Self> {
        let worker_url = std::env::var("R2_WORKER_URL")?;
        let token = std::env::var("R2_TOKEN")?;
        let client = Client::new();
        Ok(R2Client {
            client,
            worker_url,
            token,
        })
    }

    pub async fn upload_file(&self, download_url: &str, name: &str) -> Result<()> {
        let endpoint = format!("{}/fetchSave", self.worker_url);
        let request_body = UploadRequest {
            url: download_url.to_string(),
            name: name.to_string(),
        };

        let response = self
            .client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request_body)
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        if status.is_success() {
            info!("Upload success: {} {}", status.as_u16(), text);
            Ok(())
        } else {
            warn!("Upload failed: {} {}", status.as_u16(), text);
            Err(anyhow!("Upload failed: {} {}", status.as_u16(), text))
        }
    }

    pub async fn delete_file(&self, name: &str) -> Result<()> {
        let endpoint = format!("{}/delete", self.worker_url);
        let request_body = UploadRequest {
            url: "".to_string(),
            name: name.to_string(),
        };

        let response = self
            .client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request_body)
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        if status.is_success() {
            info!("Delete success: {} {}", status.as_u16(), text);
            Ok(())
        } else {
            warn!("Delete failed: {} {}", status.as_u16(), text);
            Err(anyhow!("Delete failed: {} {}", status.as_u16(), text))
        }
    }
}

#[cfg(test)]
mod tests {

    use assertables::assert_contains;

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
        assert_contains!(
            result.unwrap_err().to_string(),
            "Failed to download the content"
        );
    }

    #[tokio::test]
    async fn test_upload_file_already_exist() {
        let client = R2Client::try_from_env().unwrap();
        let download_url = VALID_URL;
        let name = "test_exist.png";
        let result = client.upload_file(download_url, name).await;
        assert!(result.is_err());
        assert_contains!(result.unwrap_err().to_string(), "already exists");
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
        assert_contains!(result.unwrap_err().to_string(), "not exist");
    }
}
