use super::models;
use super::services;



#[tauri::command]
pub async fn process_images(
    image_data: String,
    app_handle: tauri::AppHandle,
) -> Result<models::ONNXResponse, String> {
    let response: models::ONNXResponse = services::process_images_service(image_data, app_handle)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}
