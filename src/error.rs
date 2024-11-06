use reqwest::Response;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LfaError {
    #[error("Request failed: {0}")]
    RequestMiddleWareError(#[from] reqwest_middleware::Error),
    #[error("Serialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Upload failed with status {0}: {1}")]
    UploadFailed(u16, String),

    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    // fetch client
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Bad request: {0}")]
    BadRequest(String), // 400
    #[error("Unauthorized: {0}")]
    Unauthorized(String), // 401
    #[error("File not exists: {0}")]
    FileNotExists(String), // 404
    #[error("File already exists: {0}")]
    FileExists(String), // 409
    #[error("Internal server error: {0}")]
    InternalServerError(String), // 500
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(u16, String),
}

impl LfaError {
    pub async fn from_response(response: Response) -> Self {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        match status.as_u16() {
            400 => LfaError::BadRequest(text),
            401 => LfaError::Unauthorized(text),
            404 => LfaError::FileNotExists(text),
            409 => LfaError::FileExists(text),
            500 => LfaError::InternalServerError(text),
            _ => LfaError::UnexpectedStatusCode(status.as_u16(), text),
        }
    }
}
