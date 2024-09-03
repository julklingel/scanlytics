use chrono::Utc;
use std::fs;
use std::path::Path;
use surreal::sql::Thing;
use surreal::Surreal;
use tauri::api::path::app_local_data_dir;

#[tauri::command]
pub async fn create_report(
    db: tauri::State<'_, Surreal<Client>>,
    report_request: ReportRequest,
) -> Result<(), String> {
    println!("Creating report: {:?}", report_request);

    // Get the app's local data directory
    let app_data_dir = app_local_data_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get app data directory".to_string())?;

    // Create a 'reports' subdirectory if it doesn't exist
    let reports_dir = app_data_dir.join("reports");
    fs::create_dir_all(&reports_dir)
        .map_err(|e| format!("Failed to create reports directory: {}", e))?;

    let mut image_records = Vec::new();

    // Save each file and create image records
    for file in report_request.files {
        let image_id = NewId::<ImageRecord>::new();
        let file_extension = Path::new(&file.filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        let new_filename = format!("{}.{}", image_id, file_extension);
        let file_path = reports_dir.join(&new_filename);

        // Save the file
        fs::write(&file_path, &file.content)
            .map_err(|e| format!("Failed to write file {}: {}", new_filename, e))?;

        // Create image record
        let image_record = ImageRecord {
            id: image_id,
            name: new_filename.clone(),
            path: file_path.to_str().unwrap().to_string(),
            patient: report_request.patient_id.clone(),
            file_type: file_extension.to_string(),
            body_type: report_request.body_type.clone(),
            modality_type: report_request.modality_type.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Insert image record into database
        let created_image: ImageRecord = db
            .create("Image")
            .content(&image_record)
            .await
            .map_err(|e| format!("Failed to create image record: {}", e))?;

        image_records.push(created_image.id);
    }

    // Create report record
    let report_record = ReportRecord {
        id: NewId::<ReportRecord>::new(),
        patient: report_request.patient_id,
        user_owner: report_request.user_owner,
        report_text: report_request.report_text,
        images: image_records,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Insert report record into database
    let created_report: ReportRecord = db
        .create("Report")
        .content(&report_record)
        .await
        .map_err(|e| format!("Failed to create report record: {}", e))?;

    // Update Patient record
    db.query("UPDATE type::thing($table, $id) SET reports += $report")
        .bind(("table", "Patient"))
        .bind(("id", &report_record.patient))
        .bind(("report", &created_report.id))
        .await
        .map_err(|e| format!("Failed to update Patient record: {}", e))?;

    // Update User record
    db.query("UPDATE type::thing($table, $id) SET reports += $report")
        .bind(("table", "User"))
        .bind(("id", &report_record.user_owner))
        .bind(("report", &created_report.id))
        .await
        .map_err(|e| format!("Failed to update User record: {}", e))?;

    Ok(())
}
