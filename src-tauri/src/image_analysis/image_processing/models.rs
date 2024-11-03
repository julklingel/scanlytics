use serde::{Deserialize, Serialize};
use crate::image_analysis::ml_models::models::ModelConfig;
use tract_onnx::prelude::RunnableModel;

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

#[derive(Debug)]
pub struct ImageProcessor {
    pub(crate) model: RunnableModel<tract_onnx::prelude::TypedFact, Box<dyn tract_onnx::prelude::TypedOp>, tract_onnx::prelude::Graph<tract_onnx::prelude::TypedFact, Box<dyn tract_onnx::prelude::TypedOp>>>,
    pub(crate) config: ModelConfig,
}


