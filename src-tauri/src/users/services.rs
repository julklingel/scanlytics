use super::models;
use scanlytics_db::{Surreal, Any};



pub async fn get_users_service(db: &Surreal<Any>) -> Result<Vec<models::UserResponse>, String> {
    let records: Vec<models::UserResponse> = db
        .select("User")
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}


pub async fn create_user_service(user_record: models::UserRecord, db: &Surreal<Any>) -> Result<models::UserResponse, String> {
    let user: models::UserResponse = db
        .create("User")
        .content(user_record)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to create patient".to_string())?;

    Ok(user)

}
