
use super::models::{ApiResponse, LoginResponse, ResetPasswordRequest};
use super::services;

#[tauri::command]
pub async fn login(login_data: String) -> Result<ApiResponse<LoginResponse>, String> {
    services::login_service(login_data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reset_password(
    reset_data: String,
) -> Result<ApiResponse<ResetPasswordRequest>, String> {
    services::reset_password_service(reset_data)
        .await
        .map_err(|e| e.to_string())
}
