use super::models::{Record, Testdata};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn test_db_service_write(
    db: &Surreal<Client>,
    data: Testdata,
) -> Result<Vec<Record>, String> {
    let created: Vec<Record> = db
        .create("test_db")
        .content(data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(created)
}

pub async fn test_db_service_read(
    db: &Surreal<Client>,
    id: Option<String>,
) -> Result<Vec<Record>, String> {
    match id {
        Some(id) => {
            let read: Option<Record> = db
                .select(("test_db", &id))
                .await
                .map_err(|e| e.to_string())?;
            Ok(read.map(|r| vec![r]).unwrap_or_default())
        }
        None => {
            let read: Vec<Record> = db.select("test_db").await.map_err(|e| e.to_string())?;
            Ok(read)
        }
    }
}

pub async fn test_db_service_delete(
    db: &Surreal<Client>,
    id: String,
) -> Result<Option<Record>, String> {
    let deleted: Option<Record> = db
        .delete(("test_db", &id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}
