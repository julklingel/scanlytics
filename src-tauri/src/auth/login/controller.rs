
use super::models::{ApiResponse, LoginResponse};
use super::services;

/// This function is the command controller that logs a user in.
/// # Arguments
/// 
/// * `login_data` - A JSON string containing the login credentials.
/// * `base_url` - An optional base URL for the API endpoint.
///
/// # Returns
///
/// This function returns a `Result` containing either an `ApiResponse<LoginResponse>`
/// on success or a `String` error message on failure.
#[tauri::command]
pub async fn login(login_data: String, base_url: Option<String>) -> Result<ApiResponse<LoginResponse>, String> {
    services::login_service(login_data, base_url)
        .await
        .map_err(|e| e.to_string())
}