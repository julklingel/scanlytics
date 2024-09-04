use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn create_report(
    db: State<'_, RwLock<Surreal<Client>>>,
    report_request: String,
    app_handle: tauri::AppHandle,
) -> Result<models::ReportResponse, String> {
    let report_request: models::ReportRequest = serde_json::from_str(&report_request)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;

    let db = db.write().await;
    let response: models::ReportResponse =
        services::create_report_service(&db, report_request, app_handle).await?;

    Ok(response)
}
