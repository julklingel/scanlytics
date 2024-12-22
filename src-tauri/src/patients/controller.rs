use super::models;
use super::services;

use scanlytics_db::DbConnection;
use tauri::State;


/// Creates a new patient record in the system.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `patient_request` - JSON string containing patient data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientResponse)` - Successfully created patient record
/// * `Err(String)` - Error message if creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * The patient request JSON is invalid
/// * Database operations fail
/// * Required relationships cannot be established

#[tauri::command]
pub async fn create_patient(
    db_connection: State<'_, DbConnection>,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    let db = db_connection.get().lock().await;
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;
    
    let response: models::PatientResponse =
        services::create_patient_service(&db, patient_request).await?;
    Ok(response)
}

/// Retrieves all patient records from the system.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<PatientResponse>)` - List of all patient records
/// * `Err(String)` - Error message if retrieval fails

#[tauri::command]
pub async fn get_patients(
    db_connection: State<'_, DbConnection>,
) -> Result<Vec<models::PatientResponse>, String> {
    let db = db_connection.get().lock().await;
    let response: Vec<models::PatientResponse> = services::get_patient_service(&db)
        .await?
        .into_iter()
        .map(|record| models::PatientResponse {
            id: record.id,
            name: record.name,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            notes: record.notes,
            reports: record.reports,
            images: record.images,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
        .collect();
    Ok(response)
}


/// Updates an existing patient record.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `id` - Unique identifier of the patient to update
/// * `patient_request` - JSON string containing updated patient data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientResponse)` - Updated patient record
/// * `Err(String)` - Error message if update fails
///
/// # Errors
///
/// This function will return an error if:
/// * The patient request JSON is invalid
/// * The specified patient ID doesn't exist
/// * Database operations fail

#[tauri::command]
pub async fn update_patient(
    db_connection: State<'_, DbConnection>,
    id: String,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    let db = db_connection.get().lock().await;
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;

   
    let updated_record = services::update_patient_service(&db, id, patient_request).await?;

    if let Some(record) = updated_record {
        let response = models::PatientResponse {
            id: record.id,
            name: record.name,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            notes: record.notes,
            reports: record.reports,
            images: record.images,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };

        Ok(response)
    } else {
        Err("No record updated".to_string())
    }
}


/// Deletes a patient record from the system.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `id` - Unique identifier of the patient to delete
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientResponse)` - Deleted patient record
/// * `Err(String)` - Error message if deletion fails
///
/// # Errors
///
/// This function will return an error if:
/// * The specified patient ID doesn't exist
/// * Database operations fail

#[tauri::command]
pub async fn delete_patient(
    db_connection: State<'_, DbConnection>,
    id: String,
) -> Result<models::PatientResponse, String> {
    let db = db_connection.get().lock().await;
    let deleted_record = services::delete_patient_service(&db, id).await?;

    if let Some(record) = deleted_record {
        let response = models::PatientResponse {
            id: record.id,
            name: record.name,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            notes: record.notes,
            reports: record.reports,
            images: record.images,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };

        Ok(response)
    } else {
        Err("No record deleted".to_string())
    }
}
