use super::models;
use super::services;

use tauri::State;
use scanlytics_db::DbConnection;




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
