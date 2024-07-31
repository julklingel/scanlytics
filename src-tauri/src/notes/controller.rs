use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn create_patient_note(
    db: State<'_, RwLock<Surreal<Client>>>,
    patient_note_request: models::PatientNoteRequest,
) -> Result<models::PatientNoteResponse, String> {
    let db = db.write().await;
    let mut records = services::create_patient_note_service(&db, patient_note_request).await?;
    if records.is_empty() {
        return Err("No record created".to_string());
    }
    let record = records.pop().unwrap();
    let response = models::PatientNoteResponse {
        id: format!("{}:{}", record.id.tb, record.id.id),
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
    println!("Created: {:?}", response);
    Ok(response)
}