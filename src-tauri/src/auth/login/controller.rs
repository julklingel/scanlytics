
use super::models::{ApiResponse, LoginResponse};
use super::services;

/// Tauri command for user authentication.
///
/// This command handles the login process by:
/// 1. Accepting login credentials
/// 2. Authenticating with the backend server
/// 3. Managing authentication tokens
/// 4. Providing response to the frontend
///
/// # Arguments
///
/// * `login_data` - JSON string containing:
///   * `user_email`: User's email address
///   * `user_password`: User's password
/// * `base_url` - Optional base URL for the authentication server
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(ApiResponse<LoginResponse>)` - Successful login with token
/// * `Err(String)` - Error message if login fails
///
/// # Security
///
/// - Credentials are transmitted securely
/// - Tokens are stored in the system keyring
/// - Error messages are sanitized

#[tauri::command]
pub async fn login(login_data: String, base_url: Option<String>) -> Result<ApiResponse<LoginResponse>, String> {
    services::login_service(login_data, base_url)
        .await
        .map_err(|e| e.to_string())
}