
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ Testdata, Record};


pub async fn test_db_service(db: &Surreal<Client>) -> Result<Vec<Record>, String> {
    db.create("test_db")
        .content(Testdata {
            txt1: String::from("This is sample txt"),
            bool1: true
        })
        .await
        .map_err(|e| e.to_string())
}