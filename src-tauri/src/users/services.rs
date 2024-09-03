use super::models::UserResponse;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn get_users_service(db: &Surreal<Client>) -> Result<Vec<UserResponse>, String> {
    let records: Vec<UserResponse> = db.select("User").await.map_err(|e| e.to_string())?;
    Ok(records)
}
