use super::models;
use super::services;

use tauri::State;
use scanlytics_db::DbConnection;



/// Creates a new patient note in the system.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `patient_note_request` - JSON string containing note data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientNoteResponse)` - Successfully created note
/// * `Err(String)` - Error message if creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * The note request JSON is invalid
/// * Database operations fail
/// * Patient or user references are invalid
#[tauri::command]
pub async fn create_patient_note(
    db_connection: State<'_, DbConnection>,
    patient_note_request: String,
) -> Result<models::PatientNoteResponse, String> {
    let db = db_connection.get().lock().await;
    let patient_note_request: models::PatientNoteRequest =
        serde_json::from_str(&patient_note_request)
            .map_err(|e| format!("Failed to parse patient note request: {}", e))?;

    let  note: models::PatientNoteResponse =
        services::create_patient_note_service(&db, patient_note_request).await?;

    Ok(note)
}

/// Retrieves all patient notes with associated patient information.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<PatientNoteWithPatientResponse>)` - List of notes with patient details
/// * `Err(String)` - Error message if retrieval fails
#[tauri::command]
pub async fn get_patient_notes(
    db_connection: State<'_, DbConnection>
) -> Result<Vec<models::PatientNoteWithPatientResponse>, String> {
    let db = db_connection.get().lock().await;
    let response = services::get_patient_notes_service(&db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

/// Updates an existing patient note.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `id` - Unique identifier of the note to update
/// * `patient_note_request` - JSON string containing updated note data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientNoteResponse)` - Updated note record
/// * `Err(String)` - Error message if update fails
///
/// # Errors
///
/// This function will return an error if:
/// * The note request JSON is invalid
/// * The specified note ID doesn't exist
/// * Database operations fail

#[tauri::command]
pub async fn update_patient_note(
    db_connection: State<'_, DbConnection>,
    id: String,
    patient_note_request: String,
) -> Result<models::PatientNoteResponse, String> {
    let db = db_connection.get().lock().await;

    let patient_note_request: models::PatientNoteRequest =
        serde_json::from_str(&patient_note_request)
            .map_err(|e| format!("Failed to parse patient note request: {}", e))?;

    let updated_record =
        services::update_patient_note_service(&db, id, patient_note_request).await?;

    if let Some(record) = updated_record {
        let response = models::PatientNoteResponse {
            id: record.id,
            patient: record.patient,
            symptoms: record.symptoms,
            diagnosis: record.diagnosis,
            treatment: record.treatment,
            is_urgent: record.is_urgent,
            severity: record.severity,
            user_owner: record.user_owner,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };

        Ok(response)
    } else {
        Err("No record updated".to_string())
    }
}


/// Deletes a patient note from the system.
///
/// # Arguments
///
/// * `db_connection` - Database connection state
/// * `id` - Unique identifier of the note to delete
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientNoteResponse)` - Deleted note record
/// * `Err(String)` - Error message if deletion fails

#[tauri::command]
pub async fn delete_patient_note(
    db_connection: State<'_, DbConnection>,
    id: String,
) -> Result<models::PatientNoteResponse, String> {
    let db = db_connection.get().lock().await;
    let deleted_record = services::delete_patient_note_service(&db, id).await?;

    if let Some(record) = deleted_record {
        let response = models::PatientNoteResponse {
            id: record.id,
            patient: record.patient,
            symptoms: record.symptoms,
            diagnosis: record.diagnosis,
            treatment: record.treatment,
            is_urgent: record.is_urgent,
            severity: record.severity,
            user_owner: record.user_owner,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };

        Ok(response)
    } else {
        Err("No record deleted".to_string())
    }
}
