use super::services;


/// Tauri command for validating authentication tokens.
///
/// This command handles the token validation process by:
/// 1. Retrieving stored token
/// 2. Validating with backend server
/// 3. Storing renewed token if provided
///
/// # Arguments
///
/// * `user_email` - Email address associated with the token
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(())` - Token is valid and renewed if necessary
/// * `Err(String)` - Error message if validation fails


#[tauri::command]
pub async fn validate_token(user_email: String) -> Result<(), String> {
    services::validate_token_service(&user_email)
        .await
        .map_err(|e| e.to_string())
}
