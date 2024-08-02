use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn create_patient_note(
    db: State<'_, RwLock<Surreal<Client>>>,
    patient_note_request: String,
) -> Result<models::PatientNoteResponse, String> {
    let patient_note_request: models::PatientNoteRequest = serde_json::from_str(&patient_note_request)
        .map_err(|e| format!("Failed to parse patient note request: {}", e))?;
    let db = db.write().await;
    let mut records = services::create_patient_note_service(&db, patient_note_request).await?;
    if records.is_empty() {
        return Err("No record created".to_string());
    }
    let record = records.pop().unwrap();
    
    let response = models::PatientNoteResponse {
        id: record.id,
        patient_name: record.patient_name,
        patient_id: record.patient_id,
        symptoms: record.symptoms,
        diagnosis: record.diagnosis,
        treatment: record.treatment,
        is_urgent: record.is_urgent,
        department: record.department,
        attending_doctor: record.attending_doctor,
        severity: record.severity,
        created_at: record.created_at
    };
    
    Ok(response)
}

#[tauri::command]
pub async fn get_patient_notes(
    db: State<'_, RwLock<Surreal<Client>>>,
) -> Result<Vec<models::PatientNoteResponse>, String> {
    let db = db.write().await;
    let records = services::get_patient_notes_service(&db).await?;
    let response = records
        .iter()
        .map(|record| models::PatientNoteResponse {
            id: record.id.clone(),
            patient_name: record.patient_name.clone(),
            patient_id: record.patient_id.clone(),
            symptoms: record.symptoms.clone(),
            diagnosis: record.diagnosis.clone(),
            treatment: record.treatment.clone(),
            is_urgent: record.is_urgent,
            department: record.department.clone(),
            attending_doctor: record.attending_doctor.clone(),
            severity: record.severity.clone(),
            created_at: record.created_at.clone()
        })
        .collect();
    Ok(response)
}

#[tauri::command]
pub async fn update_patient_note(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
    patient_note_request: String,
) -> Result<models::PatientNoteResponse, String> {
    println!("update_patient_note: id: {}, patient_note_request: {}", id, patient_note_request);
    let patient_note_request: models::PatientNoteRequest = serde_json::from_str(&patient_note_request)
        .map_err(|e| format!("Failed to parse patient note request: {}", e))?;
    
    let db = db.write().await;
    let updated_record = services::update_patient_note_service(&db, id, patient_note_request).await?;
    
    if let Some(record) = updated_record {
        let response = models::PatientNoteResponse {
            id: record.id,
            patient_name: record.patient_name,
            patient_id: record.patient_id,
            symptoms: record.symptoms,
            diagnosis: record.diagnosis,
            treatment: record.treatment,
            is_urgent: record.is_urgent,
            department: record.department,
            attending_doctor: record.attending_doctor,
            severity: record.severity,
            created_at: record.created_at
        };
        
        Ok(response)
    } else {
        Err("No record updated".to_string())
    }
}


#[tauri::command]
pub async fn delete_patient_note(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
) -> Result<models::PatientNoteResponse, String> {
    let db = db.write().await;
    let deleted_record = services::delete_patient_note_service(&db, id).await?;
    
    if let Some(record) = deleted_record {
        let response = models::PatientNoteResponse {
            id: record.id,
            patient_name: record.patient_name,
            patient_id: record.patient_id,
            symptoms: record.symptoms,
            diagnosis: record.diagnosis,
            treatment: record.treatment,
            is_urgent: record.is_urgent,
            department: record.department,
            attending_doctor: record.attending_doctor,
            severity: record.severity,
            created_at: record.created_at
        };
        
        Ok(response)
    } else {
        Err("No record deleted".to_string())
    }
}