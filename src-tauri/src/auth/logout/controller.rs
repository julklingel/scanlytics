use super::services;



/// Tauri command for user logout.
///
/// This command handles the logout process by:
/// 1. Removing stored authentication tokens
/// 2. Cleaning up the user session
/// 3. Providing status feedback
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(String)` - Success message indicating logout status:
///   - "Successfully logged out" - When logout was successful
///   - "No active session found" - When no active session existed
/// * `Err(String)` - Error message if logout fails
///
/// # Error Messages
///
/// Error messages may indicate:
/// - Keyring access failures
/// - Credential deletion failures
/// - System-level errors
///
/// # Security
///
/// - Ensures complete removal of authentication data
/// - Handles edge cases gracefully
/// - Provides clear operation status

#[tauri::command]
pub async fn logout() -> Result<String, String> {
    services::logout_service()
        .await
        .map_err(|e| e.to_string())
}