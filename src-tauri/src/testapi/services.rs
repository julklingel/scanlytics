
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ Testdata, Record};



pub async fn test_db_service(db: &Surreal<Client>, data: Testdata) -> Result<Vec<Record>, String> {
    let created: Vec<Record> = db
        .create("test_db")
        .content(data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(created)
}