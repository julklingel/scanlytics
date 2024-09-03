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
) -> Result<Vec<models::ReportResponse>, String> {
    println!("Obeject before serde: {}", report_request);
    let report_request: models::ReportRequest = serde_json::from_str(&report_request)
    .map_err(|e| format!("Failed to parse patient note request: {}", e))?;

    println!("Object after serde: {:?}", report_request);

    let db = db.write().await;
    let response: Vec<models::ReportResponse> =
        services::create_report_service(&db, report_request).await?;

    Ok(response)
}
