use serde::{Deserialize, Serialize};
use thiserror::Error;
use reqwest::Client;


#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub filename: String,
    pub extension: String,
    pub data: Vec<u8>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub filename: String,
    pub image_type: String,
    pub confidence: f32,
}



#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("File system error: {0}")]
    FileSystem(String),
    
    #[error("Model processing error: {0}")]
    Processing(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Image processing error: {0}")]
    Image(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[derive(Debug)]
pub struct ModelConfig {
    pub input_shape: (usize, usize),
    pub class_mapping: Vec<&'static str>,
}

#[derive(Debug)]
pub struct ModelManager {
    pub client: Client,
    pub app_handle: tauri::AppHandle,
}
