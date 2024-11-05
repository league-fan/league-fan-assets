use anyhow::{anyhow, Result};
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
    pub fn from_env() -> Self {
        let worker_url = std::env::var("R2_WORKER_URL").expect("R2_WORKER_URL is not set");
        let token = std::env::var("R2_TOKEN").expect("R2_TOKEN is not set");
        let client = Client::new();
        R2Client {
            client,
            worker_url,
            token,
        }
    }

    pub async fn upload_file(&self, download_url: &str, name: &str) -> Result<()> {
        let request_body = UploadRequest {
            url: download_url.to_string(),
            name: name.to_string(),
        };

        let response = self
            .client
            .post(&self.worker_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request_body)
            .send()
            .await?;
        eprintln!("Response: {:#?}", response);
        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to upload file"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_file() {
        let client = R2Client::from_env();
        let download_url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/assets/loot/loottable_chest_generic_1.png";
        let name = "/lol-game-data/assets/ASSETS/Loot/loottable_chest_generic_1.png";
        let result = client.upload_file(download_url, name).await;
        assert!(result.is_ok());
    }
}
