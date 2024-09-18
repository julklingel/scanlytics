
use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn login(
    db: State<'_, RwLock<Surreal<Client>>>,
    login_data: String,
) -> Result<models::LoginResponse, String> {

    
    let db = db.read().await;
    let response = services::login_service(&db, login_data).await?;



    Ok(response)
}