use std::sync::Arc;

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::{client::client_trait::AssetsTask, client::fetch_client::FetchClient};

use super::utils::{get_assets_url, AssetsTypeTrait, Config};

pub trait ToTasks {
    fn to_tasks(&self, config: Arc<Config>) -> Vec<AssetsTask>;
}

pub trait CollecTasks {
    fn collect_tasks(&self, config: Arc<Config>) -> Vec<AssetsTask>;
}

pub trait FilterEmptyAssets: Sized {
    fn filter_empty_assets(&self) -> Option<Self>;
}

impl FilterEmptyAssets for String {
    fn filter_empty_assets(&self) -> Option<Self> {
        match self.trim_start_matches("/lol-game-data/assets/").is_empty() {
            true => None,
            false => Some(self.clone()),
        }
    }
}

pub trait FromUrl: DeserializeOwned + AssetsTypeTrait {
    fn from_url(config: &Config) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        Self: Sync,
    {
        async {
            let assets_type = Self::assets_type(); // 使用 Self 调用关联函数
            let url = get_assets_url(&assets_type, &config.language, &config.version);
            FetchClient::default()
                .get(&url)
                .await?
                .json::<Self>() // 明确指定反序列化的目标类型
                .await
                .map_err(|e| e.into())
        }
    }
}
