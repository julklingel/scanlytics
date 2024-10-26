use crate::auth::validate::services::auth_middleware;
use super::models;
use super::services;

use crate::db::models::DbConnection;
use tauri::State;

#[tauri::command]
pub async fn create_report(
    db_connection: State<'_, DbConnection>,
    report_request: String,
    app_handle: tauri::AppHandle,
) -> Result<models::CreateReportResponse, String> {
    let db = db_connection.get().lock().await;
    let report_request: models::ReportRequest = serde_json::from_str(&report_request)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;
   
    let response: models::CreateReportResponse =
        services::create_report_service(&db, report_request, app_handle).await?;

    Ok(response)
}

#[tauri::command]
pub async fn get_reports(
    db_connection: State<'_, DbConnection>,
    username: String,
) -> Result<Vec<models::ReportResponse>, String> {
    auth_middleware(&username, || async {
        let db = db_connection.get().lock().await;
        services::get_reports_service(&db)
            .await
            .map_err(|e| e.to_string())
    }).await
}


#[tauri::command]
pub async fn get_report_images(
    db_connection: State<'_, DbConnection>,
    report_id: String,
) -> Result<Vec<models::ImageInfo>, String> {
    let db = db_connection.get().lock().await;
    let response: Vec<models::ImageInfo> = services::get_report_images_service(&db, report_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}
