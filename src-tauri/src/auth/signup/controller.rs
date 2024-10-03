
use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;


#[tauri::command]
pub async fn signup(
    db: State<'_, RwLock<Surreal<Client>>>,
    signup_data: String,
) -> Result<models::SignupResponse, String> {
    let db = db.read().await;
    let response = services::signup_service(&db, signup_data).await?;
    Ok(response)
}