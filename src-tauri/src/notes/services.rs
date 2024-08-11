
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ PatientNoteRecord, PatientNoteRequest, PatientNoteResponse}; 
use surrealdb::sql::Thing;

pub async fn create_patient_note_service(
    db: &Surreal<Client>,
    data: PatientNoteRequest,
) -> Result<Vec<PatientNoteResponse>, String> {
  
    let patient: Option<Thing> = db
        .select(("Patient", &data.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;


    let patient_note_record = PatientNoteRecord {
        patient,
        symptoms: data.symptoms,
        diagnosis: data.diagnosis,
        treatment: data.treatment,
        severity: data.severity,
        is_urgent: data.is_urgent,
        user_owner: data.user_owner,  
    };

   
    let created: Vec<PatientNoteResponse> = db
        .create("PatientNote")
        .content(patient_note_record)
        .await
        .map_err(|e| e.to_string())?;

    Ok(created)
}


pub async fn get_patient_notes_service(
    db: &Surreal<Client>
) -> Result<Vec<PatientNoteResponse>, String> {
  
    let patient_notes: Vec<PatientNoteResponse> = db
        .select("PatientNote")
        .await
        .map_err(|e| format!("Failed to fetch patient notes: {}", e))?;

    let mut sorted_notes = patient_notes;
    sorted_notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(sorted_notes)
}






pub async fn update_patient_note_service(
    db: &Surreal<Client>,
    id: String,
    data: PatientNoteRequest
) -> Result<Option<PatientNoteResponse>, String> {
   
    let patient: Option<Thing> = db
        .select(("Patient", &data.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

    
    let updated_note = PatientNoteRecord {
        patient,
        symptoms: data.symptoms,
        diagnosis: data.diagnosis,
        treatment: data.treatment,
        severity: data.severity,
        is_urgent: data.is_urgent,
        user_owner: data.user_owner,  
    };


    let updated: Option<PatientNoteResponse> = db
        .update(("PatientNote", id))
        .merge(updated_note)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated)
}


pub async fn delete_patient_note_service(db: &Surreal<Client>, id: String) -> Result<Option<PatientNoteResponse>, String> {
    let deleted: Option<PatientNoteResponse> = db
        .delete(("PatientNote", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}