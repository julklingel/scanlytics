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

    let mut image_ids = Vec::new();

    let save_dir = Path::new("saved_images");
    if !save_dir.exists() {
        fs::create_dir_all(save_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    for file in &report_request.files {
        let image_request = models::ImageRequest {
            name: file.filename.clone(),
            path: String::new(), // We'll update this after saving the file
            patient: patient.id.clone(),
            user: user_owner.id.clone(),
            file_type: file.extension.clone(),
            modal_type: "xray".to_string(), // You might want to make this dynamic
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

        // Now that we have the ID, let's save the file
        let file_name = format!("{}.{}", created_image.id, file.extension);
        let file_path = save_dir.join(&file_name);

        // Convert Vec<u8> to DynamicImage
        let image = image::load_from_memory(&file.data)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        // Save the image
        image.save(&file_path)
            .map_err(|e| format!("Failed to save image: {}", e))?;

        // Update the image record with the correct path
        let file_path_str = file_path.to_str().unwrap().to_string();
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
        files: image_ids,
    };

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
