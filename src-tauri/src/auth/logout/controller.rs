use super::services;

#[tauri::command]
pub async fn logout() -> Result<String, String> {
    services::logout_service()
        .await
        .map_err(|e| e.to_string())
}