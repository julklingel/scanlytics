use super::models::LogoutError;
use keyring::Entry;

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
