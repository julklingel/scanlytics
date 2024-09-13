use super::models;
use super::services;


#[tauri::command]
pub async fn test_onnx_model(
    image_path: String,
) -> Result<models::ONNXResponse, String> {
    println!("Testing ONNX model with image: {}", image_path);
    let response: models::ONNXResponse = services::test_onnx_model_service(image_path)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}
