use super::models::{SignupError, SignupResponse};
use super::services;
use tauri::State;
use crate::db::models::DbConnection;

#[tauri::command]
pub async fn signup(
    db_connection: State<'_, DbConnection>,
    signup_data: String,
) -> Result<SignupResponse, String> {
    let db = db_connection.get().lock().await;
    
    services::signup_service(&db, signup_data)
        .await
        .map_err(|e| e.to_string())
}