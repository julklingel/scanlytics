use super::models;
use std::fs;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;


pub async fn create_report_service(
    db: &Surreal<Client>,
    report_request: models::ReportRequest,
) -> Result<Vec<models::ReportResponse>, String> {
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

    for file in &report_request.files {
        let file_extension = file.filename.split('.').last().unwrap_or("bin");
        
        let image_request = models::ImageRequest {
            name: file.filename.clone(),
            path: String::new(), 
            patient: patient.id.to_string(),
            file_type: file_extension.to_string(),
            modality_type: "xray".to_string(), 
        };
        

        let created_image: Vec<models::ImageRecord> = db
            .create("Image")
            .content(&image_request)
            .await
            .map_err(|e| e.to_string())?;

        let image_record = created_image.first().ok_or_else(|| "Failed to create image record".to_string())?;

        let file_name = format!("{}.{}", image_record.id, file_extension);
        let file_path = format!("saved_images/{}", file_name);

        db.query("UPDATE type::thing($table, $id) SET path = $path")
            .bind(("table", "Image"))
            .bind(("id", &image_record.id))
            .bind(("path", &file_path))
            .await
            .map_err(|e| e.to_string())?;

        fs::write(&file_path, &file.content).map_err(|e| e.to_string())?;
        image_ids.push(image_record.id.clone());
    }

    let report_record = models::ReportRecord {
        patient: patient.id,
        user_owner: user_owner.id,
        report_text: report_request.report_text,
        files: image_ids,
    };

    let created_report: Vec<models::ReportResponse> = db
        .create("Report")
        .content(report_record)
        .await
        .map_err(|e| e.to_string())?;



    Ok(created_report)
}
