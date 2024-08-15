
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ PatientNoteRecord, PatientNoteRequest, PatientNoteResponse, PatientNoteWithPatientResponse}; 
use surrealdb::sql::Thing;
use surrealdb::Error as SurrealError;


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
) -> Result<Vec<PatientNoteWithPatientResponse>, SurrealError> {
    let query = "
        SELECT
            id,
            symptoms,
            diagnosis,
            treatment,
            severity,
            is_urgent,
            { id: patient.id, name: patient.name } as patient,
            { id: user_owner.id, name: user_owner.name } as user_owner,
            created_at,
            updated_at
        FROM PatientNote
        FETCH patient, user_owner;
    ";
    let result: Vec<PatientNoteWithPatientResponse> = db.query(query).await?.take(0)?;
    Ok(result)
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