use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use super::models::{Name, Person, Record};
use tokio::sync::RwLock;
use tauri::State;

#[tauri::command]
pub async fn create_person(db: State<'_, RwLock<Surreal<Client>>>) -> Result<String, String> {
    let db = db.write().await;
    let created: Vec<Record> = db
        .create("test_db")
        .content(Person {
            title: String::from("Founder & CEO"),
            name: Name {
                first: String::from("Tobie"),
                last: String::from("Morgan Hitchcock"),
            },
            marketing: true,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(format!("Created: {:?}", created))
}