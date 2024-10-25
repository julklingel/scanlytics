use super::models;
use super::services;



#[tauri::command]
pub async fn process_images(
    image_data: String,
    user_name: String,
    model_name: String,   
    app_handle: tauri::AppHandle,

) -> Result<models::ONNXResponse, String> {
    let user_name = serde_json::from_str(&user_name)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;
    
    let model_name = serde_json::from_str(&model_name)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;

    
    let response: models::ONNXResponse = services::process_images_service(image_data, user_name, model_name, app_handle, )
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}
