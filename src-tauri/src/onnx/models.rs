use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ONNXRequest {
    pub image_path: String,
    pub model_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ONNXResponse {
    pub image_type: String,
    pub confidence: f32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub filename: String,
    pub extension: String,
    pub data: Vec<u8>,
}
