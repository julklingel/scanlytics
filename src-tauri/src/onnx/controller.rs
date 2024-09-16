use super::models;
use super::services;


#[tauri::command]
pub async fn test_onnx_model(
    image_path: String,
    model_type: String,
    app_handle: tauri::AppHandle,
) -> Result<models::ONNXResponse, String> {
    let response: models::ONNXResponse = services::test_onnx_model_service(image_path, model_type, app_handle)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}



#[tauri::command]
pub async fn process_image(
    image_data: models::ImageData,
    app_handle: tauri::AppHandle,
) -> Result<models::ImageResponse, String> {
    let response: models::ImageResponse = services::process_image_service(image_path, app_handle)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}