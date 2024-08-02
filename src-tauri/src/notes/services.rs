
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ PatientNoteRequest, PatientNoteRecord  };



pub async fn create_patient_note_service(db: &Surreal<Client>, data: PatientNoteRequest) -> Result<Vec<PatientNoteRecord>, String> {
    let created: Vec<PatientNoteRecord> = db
        .create("PatientNote")
        .content(data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(created)
}



pub async fn get_patient_notes_service(db: &Surreal<Client>) -> Result<Vec<PatientNoteRecord>, String> {
    let records: Vec<PatientNoteRecord> = db
        .select("PatientNote")
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}