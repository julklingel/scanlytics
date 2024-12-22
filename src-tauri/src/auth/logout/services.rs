use super::models::LogoutError;
use keyring::Entry;


/// Handles the logout process by removing stored authentication tokens.
///
/// This service:
/// - Attempts to access the system keyring
/// - Removes stored credentials if they exist
/// - Handles various error conditions gracefully
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(String)` - Success message indicating logout status
/// * `Err(LogoutError)` - Error details if logout fails
///
/// # Success Messages
///
/// - "Successfully logged out" - When credentials were found and removed
/// - "No active session found" - When no credentials existed
///
/// # Errors
///
/// This function can return several types of errors:
/// * `LogoutError::KeyringDelete` - Failed to delete existing credentials
/// * `LogoutError::KeyringAccess` - Failed to access the system keyring
///
/// # Security
///
/// - Ensures complete removal of authentication tokens
/// - Handles edge cases like missing credentials
pub async fn logout_service() -> Result<String, LogoutError> {
    let service_name = "com.scanlytics.dev";
    
    match Entry::new(service_name, "scanlytics") {
        Ok(entry) => {
            entry.delete_credential().map_err(|e| {
                LogoutError::KeyringDelete(format!("Failed to delete credentials: {}", e))
            })?;
            Ok("Successfully logged out".to_string())
        }
        Err(e) => {
            if e.to_string().contains("No credential found") {
                Ok("No active session found".to_string())
            } else {
                Err(LogoutError::KeyringAccess(format!("Failed to access keyring: {}", e)))
            }
        }
    }
}
