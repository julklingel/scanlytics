use super::models;
use scanlytics_db::{Surreal, Any};


/// Retrieves all users from the database.
///
/// # Arguments
///
/// * `db` - Database connection instance
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<UserResponse>)` - List of all users
/// * `Err(String)` - Error message if the database operation fails

pub async fn get_users_service(db: &Surreal<Any>) -> Result<Vec<models::UserResponse>, String> {
    let records: Vec<models::UserResponse> = db
        .select("User")
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}

/// Creates a new user in the database.
///
/// # Arguments
///
/// * `user_record` - User data to be created
/// * `db` - Database connection instance
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(UserResponse)` - Created user record
/// * `Err(String)` - Error message if creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * The database operation fails
/// * The user creation is unsuccessful
pub async fn create_user_service(user_record: models::UserRecord, db: &Surreal<Any>) -> Result<models::UserResponse, String> {
    let user: models::UserResponse = db
        .create("User")
        .content(user_record)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to create patient".to_string())?;

    Ok(user)

}
