use thiserror::Error;

#[derive(Error, Debug)]
pub enum LfaError {
    #[error("Request failed: {0}")]
    RequestMiddleWareError(#[from] reqwest_middleware::Error),
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Serialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("File already exists: {0}")]
    FileExists(String),
    #[error("File not exists: {0}")]
    FileNotExists(String),
    #[error("Upload failed with status {0}: {1}")]
    UploadFailed(u16, String),

    #[error("Io error: {0}")]
    IoError(#[from] tokio::io::Error),
}
