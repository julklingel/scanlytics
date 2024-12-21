use crate::auth::validate::services::auth_middleware;
use super::models;
use super::services;

use scanlytics_db::DbConnection;
use scanlytics_db::{Surreal, Any};
use tokio::sync::MutexGuard;

use tauri::State;


/// Creates a new medical report with associated images.
///
/// This command handles the creation of a medical report, including:
/// - Saving report details to the database
/// - Processing and storing associated images
/// - Creating relationships between reports and images
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `report_request` - JSON string containing report data and image files
/// * `app_handle` - Tauri application handle for accessing app paths
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(CreateReportResponse)` - Successfully created report with details
/// * `Err(String)` - Error message if creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * The report request JSON is invalid
/// * Database operations fail
/// * Image processing or saving fails
/// * File system operations fail

#[tauri::command]
pub async fn create_report(
    db_connection: State<'_, DbConnection>,
    report_request: String,
    app_handle: tauri::AppHandle,
) -> Result<models::CreateReportResponse, String> {
    println!("Creating report");
    let db: MutexGuard<'_, Surreal<Any>> = db_connection.get().lock().await;
    let report_request: models::ReportRequest = serde_json::from_str(&report_request)
        .map_err(|e| format!("Tauri: Failed to parse report request : {}", e))?;
   
    let response: models::CreateReportResponse =
        services::create_report_service(&db, report_request, app_handle).await?;

    Ok(response)
}


/// Retrieves all medical reports accessible to a specific user.
///
/// This endpoint is protected by authentication middleware and returns
/// all reports that the authenticated user has permission to view.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `username` - Username of the authenticated user
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<ReportResponse>)` - List of medical reports
/// * `Err(String)` - Error message if retrieval fails
///
/// # Authentication
///
/// This endpoint requires valid user authentication through the auth_middleware.
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


/// Retrieves all images associated with a specific medical report.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `report_id` - Unique identifier of the report
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<ImageInfo>)` - List of image information
/// * `Err(String)` - Error message if retrieval fails

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
