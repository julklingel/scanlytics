use serde::{Deserialize, Serialize};
use thiserror::Error;
use reqwest::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub filename: String,
    pub extension: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ONNXResponse {
    pub results: Vec<ImageResult>,
    pub statements: Vec<StatementResponse>, // Added this field
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub filename: String,
    pub image_type: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatementResponse {
    pub indication: String,
    pub statement: String,
    pub assessment: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    pub name: String,
}


#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("Server error: {status} - {message}")]
    ServerError { status: StatusCode, message: String },
}
