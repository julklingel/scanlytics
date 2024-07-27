
use tauri::State;
use tokio::sync::RwLock;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::services;
use super::models;

#[tauri::command]
pub async fn test_db(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: Option<String>,
    bool1: bool,
    txt1: String
) -> Result<models::TestdbResponse, String> {
    let db = db.write().await;
    
    let data = models::Testdata {
        id,
        bool1,
        txt1,
    };

    let created = services::test_db_service(&db, data).await?;
   
    if created.is_empty() {
        return Err("No record created".to_string());
    }

    let record = &created[0];

    let response = models::TestdbResponse {
        id: format!("{}:{}", record.id.tb, record.id.id),
        bool1: record.bool1,
        txt1: record.txt1.clone(),
    };

    println!("Created: {:?}", response);
    Ok(response)
}