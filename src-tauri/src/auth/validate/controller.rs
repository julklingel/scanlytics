use super::services;

#[tauri::command]
pub async fn validate_token(username: String) -> Result<(), String> {
    let response = services::validate_token_service(&username).await?;
    Ok(response)
}




