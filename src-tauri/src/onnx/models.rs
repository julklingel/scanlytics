use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct ONNXRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ONNXResponse {
    pub image_type: String,
    pub confidence: f32,
}
