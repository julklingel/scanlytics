
use super::models;
use super::services;

use tauri::State;
use crate::db::models::DbConnection;



#[tauri::command]
pub async fn signup(
    db_connection: State<'_, DbConnection>,
    signup_data: String,
) -> Result<models::SignupResponse, String> {
    let db = db_connection.get().lock().await;
    
    let response = services::signup_service(&db, signup_data).await?;
    Ok(response)
}