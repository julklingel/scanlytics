use super::models;
use std::fs;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use std::path::Path;


pub async fn create_report_service(
    db: &Surreal<Client>,
    report_request: models::ReportRequest,
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

    println!("Patient: {:?}", patient);
    println!("User owner: {:?}", user_owner);


    let mut image_ids = Vec::new();

    let save_dir = Path::new("saved_images");
    if !save_dir.exists() {
        fs::create_dir_all(save_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    for file in &report_request.files {
        println!("File: {:?}", file);
        let file_extension = file.filename.split('.').last().unwrap_or("bin");
        
        let image_request = models::ImageRequest {
            name: file.filename.clone(),
            path: String::new(), 
            patient: patient.id.clone(),
            user: user_owner.id.clone(),
            file_type: file_extension.to_string(),
            modal_type: "xray".to_string(), 
        };

        println!("Image request: {:?}", image_request);

        let created_image: Vec<models::ImageResponse> = db
            .create("Image")
            .content(&image_request)
            .await
            .map_err(|e| e.to_string())?;

        let created_image = created_image
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create image".to_string())?;

        println!("Created image: {:?}", created_image);

        let file_name = format!("{}.{}", created_image.id, file_extension);
        let file_path = format!("saved_images/{}", file_name);

        db.query("UPDATE type::thing($table, $id) SET path = $path")
            .bind(("table", "Image"))
            .bind(("id", &created_image.id))
            .bind(("path", &file_path))
            .await
            .map_err(|e| e.to_string())?;

        

        fs::write(&file_path, &file.extension).map_err(|e| e.to_string())?;
        image_ids.push(created_image.id);
    }

    let report_record = models::ReportRecord {
        patient: patient.id,
        user_owner: user_owner.id,
        report_text: report_request.report_text,
        files: image_ids, 
    };

    println!("Report record: {:?}", report_record);

    let created_report: Vec<models::ReportResponse> = db
        .create("Report")
        .content(report_record)
        .await
        .map_err(|e| {
            println!("Error creating report: {:?}", e);
            e.to_string()
        })?;

    let report = created_report.into_iter().next()
        .ok_or_else(|| "Failed to create report".to_string())?;

    println!("Created report: {:?}", report);

    Ok(report)
}
