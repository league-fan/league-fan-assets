use crate::{
    error::LfaError,
    preludes::{get_cdragon_url, Config, FetchClient, FALLBACK_CONFIG},
};
use serde::Serialize;

pub trait ClientTrait: Sized + Clone {
    fn default() -> Self;
    fn do_task(
        &self,
        task: &AssetsTask,
    ) -> impl std::future::Future<Output = Result<(), LfaError>> + Send;
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetsTask {
    pub url: String,
    pub fallback_url: Option<String>,
    pub path: String,
}

impl AssetsTask {
    pub fn new(url: String, fallback_url: Option<String>, path: String) -> Self {
        Self {
            url,
            fallback_url,
            path,
        }
    }

    pub fn from_path_config(path: &str, config: &Config) -> Self {
        let cdragon_url = get_cdragon_url(path, config);
        let fallback_url = get_cdragon_url(path, &FALLBACK_CONFIG);
        Self::new(cdragon_url, Some(fallback_url), path.to_string())
    }
}
