use log::info;

use super::{client_trait::{AssetsTask, ClientTrait}, fetch_client::FetchClient};

#[derive(Debug, Clone)]
pub struct DownloadClient {
    client: FetchClient,
}

impl ClientTrait for DownloadClient {
    fn default() -> Self {
        Self {
            client: FetchClient::default(),
        }
    }

    async fn do_task(
        &self,
        task: &AssetsTask,
    ) -> Result<(), crate::error::LfaError> {
        let url = task.url.clone();
        let save_path = task.path.trim_start_matches('/').to_string();

        let mut resp = self.client.get(&url).await?;
        // when get 404 not found, try fallback url
        if resp.status().as_u16() == 404 && task.fallback_url.is_some() {
            info!("File not found, try fallback: {}", url);
            resp = self
                .client
                .get(&task.fallback_url.as_ref().unwrap())
                .await?;
        }

        let bytes = resp.bytes().await?;
        tokio::fs::write(&save_path, bytes).await?;
        Ok(())
    }
}
