use super::services;

#[tauri::command]
pub async fn validate_token(user_email: String) -> Result<(), String> {
    services::validate_token_service(&user_email)
        .await
        .map_err(|e| e.to_string())
}
