// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tauri::Manager;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}


#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn crabby_print() {
    println!("Hello this is crabby");
}

async fn init_db() -> Result<Surreal<Client>, String> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.map_err(|e| e.to_string())?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| e.to_string())?;

    db.use_ns("namespace").use_db("database").await.map_err(|e| e.to_string())?;

    Ok(db)
}


async fn create_person(db: &Surreal<Client>) -> Result<String, String> {
    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(format!("Created: {:?}", created))
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db().await.expect("Failed to initialize database");
                
                // Create person at startup
                match create_person(&db).await {
                    Ok(result) => println!("{}", result),
                    Err(e) => eprintln!("Failed to create person: {}", e),
                }
                
                app.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, crabby_print])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}