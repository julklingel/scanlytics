use super::services;

#[tauri::command]
pub async fn validate_token(user_email: String) -> Result<(), String> {
    let response = services::validate_token_service(&user_email).await?;
    Ok(response)
}




