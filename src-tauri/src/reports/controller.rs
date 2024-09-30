use crate::middleware::auth::auth_middleware;
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
) -> Result<models::CreateReportResponse, String> {
    let report_request: models::ReportRequest = serde_json::from_str(&report_request)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;

    let db = db.write().await;
    let response: models::CreateReportResponse =
        services::create_report_service(&db, report_request, app_handle).await?;

    Ok(response)
}

#[tauri::command]
pub async fn get_reports(
    db: State<'_, RwLock<Surreal<Client>>>,
    username: String,
) -> Result<Vec<models::ReportResponse>, String> {
    auth_middleware(&username, || async {
        let db = db.read().await;
        services::get_reports_service(&db)
            .await
            .map_err(|e| e.to_string())
    }).await
}


#[tauri::command]
pub async fn get_report_images(
    db: State<'_, RwLock<Surreal<Client>>>,
    report_id: String,
) -> Result<Vec<models::ImageInfo>, String> {
    let db = db.read().await;
    let response: Vec<models::ImageInfo> = services::get_report_images_service(&db, report_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}
