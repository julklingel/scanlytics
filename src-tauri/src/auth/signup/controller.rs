use super::models::SignupResponse;
use super::services;
use tauri::State;
use scanlytics_db::DbConnection;

/// Tauri command for user registration.
///
/// This command handles the complete signup process by:
/// 1. Validating user input
/// 2. Checking password strength
/// 3. Registering with the backend server
/// 4. Creating local user record
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `signup_data` - JSON string containing:
///   * `full_name` - User's full name
///   * `user_email` - User's email address
///   * `password` - User's password
///   * `confirm_password` - Password confirmation
/// * `base_url` - Optional base URL for the authentication server
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(SignupResponse)` - Successful registration response
/// * `Err(String)` - Error message if registration fails
///



#[tauri::command]
pub async fn signup(
    db_connection: State<'_, DbConnection>,
    signup_data: String,
    base_url: Option<String>,
) -> Result<SignupResponse, String> {
    let db = db_connection.get().lock().await;

    
    services::signup_service(&db, signup_data, base_url)
        .await
        .map_err(|e| e.to_string())
}