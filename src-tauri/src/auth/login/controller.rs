
use super::models::{ApiResponse, LoginResponse};
use super::services;

#[tauri::command]
pub async fn login(login_data: String, base_url: Option<String>) -> Result<ApiResponse<LoginResponse>, String> {
    services::login_service(login_data, base_url)
        .await
        .map_err(|e| e.to_string())
}

