use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn test_db_write(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: Option<String>,
    bool1: bool,
    txt1: String,
) -> Result<models::TestdbResponse, String> {
    let db = db.write().await;

    let data = models::Testdata { id, bool1, txt1 };

    let created = services::test_db_service_write(&db, data).await?;

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

#[tauri::command]
pub async fn test_db_read(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: Option<String>,
) -> Result<models::TestdbResponse, String> {
    let db = db.read().await;
    let read = services::test_db_service_read(&db, id).await?;
    if read.is_empty() {
        return Err("No record found".to_string());
    }
    let record = &read[0];
    let response = models::TestdbResponse {
        id: format!("{}:{}", record.id.tb, record.id.id),
        bool1: record.bool1,
        txt1: record.txt1.clone(),
    };
    println!("Read: {:?}", response);
    Ok(response)
}

#[tauri::command]
pub async fn test_db_delete(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
) -> Result<Option<models::TestdbResponse>, String> {
    let db = db.write().await;
    let deleted = services::test_db_service_delete(&db, id).await?;

    match deleted {
        Some(record) => {
            let response = models::TestdbResponse {
                id: format!("{}:{}", record.id.tb, record.id.id),
                bool1: record.bool1,
                txt1: record.txt1.clone(),
            };
            println!("Deleted: {:?}", response);
            Ok(Some(response))
        }
        None => {
            println!("No record found to delete");
            Ok(None)
        }
    }
}
