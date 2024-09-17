use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub filename: String,
    pub extension: String,
    pub data: Vec<u8>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ONNXResponse {
    pub results: Vec<ImageResult>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub filename: String,
    pub image_type: String,
    pub confidence: f32,
}
