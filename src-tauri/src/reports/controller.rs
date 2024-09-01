use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;





#[tauri::command]
pub async fn create_report(report_request: String) -> Result<(), String> {
    let report_request: models::ReportRequest = serde_json::from_str(&report_request)
        .map_err(|e| format!("Failed to parse report request: {}", e))?;

    println!("This is the report request: {:?}", report_request);

    Ok(())
}
