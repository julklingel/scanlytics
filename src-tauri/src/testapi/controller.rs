
use tauri::State;
use tokio::sync::RwLock;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::services;


#[tauri::command]
pub async fn test_db(db: State<'_, RwLock<Surreal<Client>>>) -> Result<String, String> {
    let db = db.write().await;
    let created = services::test_db_service(&db).await?;
    
    println!("Created: {:?}", created);
    Ok(format!("Created: {:?}", created))
}