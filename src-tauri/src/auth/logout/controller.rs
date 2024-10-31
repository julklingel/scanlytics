use super::models;
use super::services;


#[tauri::command]
pub async fn logout(
) -> Result<String, String> {
    let response = services::logout_service().await?;
    Ok(response)
}

