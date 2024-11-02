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
    pub statements: Vec<StatementResponse>,
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



