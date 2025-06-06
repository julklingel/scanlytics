use super::models;
use super::services;
use tauri::State;
use scanlytics_db::DbConnection;


/// Process multiple medical images using ML models
///
/// # Arguments
///
/// * `image_data` - JSON string containing image data
/// * `user_name` - Authenticated user's name
/// * `model_name` - Name of the ML model to use
/// * `app_handle` - Tauri application handle
/// * `db_connection` - Database connection state
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(AnalysisResponse)` - Analysis results and statements
/// * `Err(String)` - Error message if processing fails
///
/// # Security
///
/// Requires valid authentication token for model access
#[tauri::command]
pub async fn process_images<'a>(
    image_data: String,
    user_name: String,
    model_name: String,   
    app_handle: tauri::AppHandle,
    db_connection: State<'_, DbConnection>,
) -> Result<models::AnalysisResponse, String> {
    let db = db_connection.get().lock().await;
    
    let user_name = serde_json::from_str(&user_name)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;
    
    let model_name = serde_json::from_str(&model_name)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;

    let response: models::AnalysisResponse = services::process_images_service(
        image_data, 
        user_name, 
        model_name, 
        app_handle,
        &db
    )
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(response)
}