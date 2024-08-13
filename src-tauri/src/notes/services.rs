
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ PatientNoteRecord, PatientNoteRequest, PatientNoteResponse, PatientNoteWithPatientResponse}; 
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
) -> Result<Vec<PatientNoteWithPatientResponse>, String> {
    println!("Watson we arrived in the notes service function");
    let query = "
        SELECT
        id,
        symptoms,
        diagnosis,
        treatment,
        severity,
        is_urgent,
        patient.name,
        user_owner.name,
        patient.id,
        user_owner.id,
        created_at,
        updated_at
        FROM PatientNote FETCH patient, user_owner;
    ";
   
    let result: Vec<PatientNoteWithPatientResponse> = db
        .query(query)
        .await
        .map_err(|e| e.to_string())?
        .take(0) // Take the first (and only) result set
        .map_err(|e| e.to_string())?;

    println!("Result from the fabulous service function: {:?}", result);
   
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