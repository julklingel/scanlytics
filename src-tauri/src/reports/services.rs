use super::models;
use std::fs;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::Manager;

use surrealdb::Error as SurrealError;

pub async fn create_report_service(
    db: &Surreal<Client>,
    report_request: models::ReportRequest,
    app_handle: tauri::AppHandle,
) -> Result<models::ReportResponse, String> {
    let patient: Option<models::PatientInfo> = db
        .select(("Patient", &report_request.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

    let user_owner: Option<models::UserInfo> = db
        .select(("User", &report_request.user_owner))
        .await
        .map_err(|e| e.to_string())?;
    let user_owner = user_owner.ok_or_else(|| "User not found".to_string())?;

    let mut image_ids = Vec::new();

    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data directory: {}", e))?;

    let save_dir = app_local_data_dir.join("saved_images");
    if !save_dir.exists() {
        fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    for file in &report_request.files {
        let image_request = models::ImageRequest {
            name: file.filename.clone(),
            path: String::new(),
            patient: patient.id.clone(),
            user: user_owner.id.clone(),
            file_type: file.extension.clone(),
            modal_type: "xray".to_string(),
        };

        let created_image: Vec<models::ImageResponse> = db
            .create("Image")
            .content(&image_request)
            .await
            .map_err(|e| e.to_string())?;

        let created_image = created_image
            .into_iter()
            .next()
            .ok_or_else(|| "Failed to create image".to_string())?;

        let file_name = format!("{}.{}", created_image.id, file.extension);
        let file_path = save_dir.join(&file_name);

        let image = image::load_from_memory(&file.data)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        image
            .save(&file_path)
            .map_err(|e| format!("Failed to save image: {}", e))?;

        let file_path_str = file_path
            .to_str()
            .ok_or_else(|| "File path contains invalid Unicode".to_string())?
            .to_string();

        db.query("UPDATE type::thing($table, $id) SET path = $path")
            .bind(("table", "Image"))
            .bind(("id", &created_image.id))
            .bind(("path", &file_path_str))
            .await
            .map_err(|e| e.to_string())?;

        image_ids.push(created_image.id);
    }

    let report_record = models::ReportRecord {
        patient: patient.id,
        user_owner: user_owner.id,
        report_text: report_request.report_text,
    };

    let created_report: Vec<models::ReportResponse> = db
        .create("Report")
        .content(report_record)
        .await
        .map_err(|e| {
            println!("Error creating report: {:?}", e);
            e.to_string()
        })?;

    let report = created_report
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create report".to_string())?;

    for image_id in image_ids {
        db.query("RELATE $image -> Images_Reports_Join -> $report")
            .bind(("image", &image_id))
            .bind(("report", &report.id))
            .await
            .map_err(|e| e.to_string())?;
    }

    println!("Created report: {:?}", report);

    Ok(report)
}

pub async fn get_reports_service(
    db: &Surreal<Client>,
) -> Result<Vec<models::ReportResponse>, SurrealError> {
    let query = "
            SELECT
                id,
                report_text,
                body_type,
                condition,
                { id: patient.id, name: patient.name } as patient,
                { id: user_owner.id, name: user_owner.name } as user_owner,
                created_at,
                updated_at
            FROM Report
            FETCH patient, user_owner;
        ";
    let result: Vec<models::ReportResponse> = db.query(query).await?.take(0)?;
    Ok(result)
}


pub async fn get_report_images_service(
    db: &Surreal<Client>,
    report_id: String,
) -> Result<Vec<models::ImageInfo>, SurrealError> {
    let query = "
        SELECT 
            image.id,
            image.path,
            image.name
        FROM Images_Reports_Join
        FETCH image := in
        WHERE out = $report_id;
    ";
    let result: Vec<models::ImageInfo> = db
        .query(query)
        .bind(("report_id", &report_id))
        .await?
        .take(0)?;
    Ok(result)
}