use std::path::Path;

use log::debug;

use crate::error::LfaError;

use super::{
    client_trait::{AssetsTask, ClientTrait},
    fetch_client::FetchClient,
};

#[derive(Debug, Clone)]
pub struct DownloadClient {
    client: FetchClient,
}

fn ensure_parent_dir_exists(path: &Path) -> std::io::Result<()> {
    let mut path = path;
    if !path.is_dir() {
        path = path.parent().unwrap();
    }
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

impl ClientTrait for DownloadClient {
    fn default() -> Self {
        Self {
            client: FetchClient::default(),
        }
    }

    async fn do_task(&self, task: &AssetsTask) -> Result<(), crate::error::LfaError> {
        let url = task.url.clone();
        let save_path = task.path.trim_start_matches('/').to_string();
        let save_path = std::path::Path::new(&save_path);

        let resp = match self.client.get(&url).await {
            Ok(resp) => resp,
            Err(LfaError::FileNotExists(_)) => {
                if task.fallback_url.is_none() {
                    return Err(LfaError::FileNotExists(url));
                }
                debug!("File not found, try fallback: {}", url);
                self.client
                    .get(&task.fallback_url.as_ref().unwrap())
                    .await?
            }
            Err(e) => return Err(e),
        };

        let bytes = resp.bytes().await?;
        ensure_parent_dir_exists(&save_path)?;
        if save_path.exists() {
            return Ok(()); // skip
        }
        tokio::fs::write(&save_path, bytes).await?;
        Ok(())
    }
}
