
use super::models::{ApiResponse, LoginResponse};
use super::services;

#[tauri::command]
pub async fn login(login_data: String) -> Result<ApiResponse<LoginResponse>, String> {
    services::login_service(login_data)
        .await
        .map_err(|e| e.to_string())
}

