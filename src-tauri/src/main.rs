// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;
mod testapi;

use db::init::init_db;
use db::api::create_person;
use testapi::controller::{test_db_write, test_db_read, test_db_delete};
use tauri::Manager;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn crabby_print() {
    println!("Hello this is crabby");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db().await.expect("Failed to initialize database");
                app.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test_db_read, test_db_delete, greet, crabby_print, create_person, test_db_write])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}