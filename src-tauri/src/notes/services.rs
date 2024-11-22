use super::models::{
    PatientNoteRecord, PatientNoteRequest, PatientNoteResponse, PatientNoteWithPatientResponse,
    PatientResponse, UserResponse,
};

use scanlytics_db::{Surreal, Db, Thing};
use scanlytics_db::Error as SurrealError;


pub async fn create_patient_note_service(
    db: &Surreal<Db>,
    data: PatientNoteRequest,
) -> Result<PatientNoteResponse, String> {
    let patient: Option<PatientResponse> = db
        .select(("Patient", &data.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

    let user_owner: Option<UserResponse> = db
        .select(("User", &data.user_owner))
        .await
        .map_err(|e| e.to_string())?;
    let user_owner = user_owner.ok_or_else(|| "User not found".to_string())?;

    let patient_note_record = PatientNoteRecord {
        patient: patient.id.clone(),
        symptoms: data.symptoms,
        diagnosis: data.diagnosis,
        treatment: data.treatment,
        severity: data.severity,
        is_urgent: data.is_urgent,
        user_owner: user_owner.id.clone(),
    };

    let note: PatientNoteResponse = db
        .create("PatientNote")
        .content(patient_note_record)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to create patient note".to_string())?;

    let patient_id = patient.id.clone();
    let note_id = note.id.clone();
    let user_owner_id = data.user_owner.clone();

  

    db.query("UPDATE type::thing($table, $id) SET notes += $note")
        .bind(("table", "Patient"))
        .bind(("id", patient_id))
        .bind(("note", note_id.clone()))
        .await
        .map_err(|e| e.to_string())?;

    db.query("UPDATE type::thing($table, $id) SET notes += $note")
        .bind(("table", "User"))
        .bind(("id", user_owner_id))
        .bind(("note", note_id))
        .await
        .map_err(|e| e.to_string())?;

    Ok(note)
}

pub async fn get_patient_notes_service(
    db: &Surreal<Db>,
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
    db: &Surreal<Db>,
    id: String,
    data: PatientNoteRequest,
) -> Result<Option<PatientNoteResponse>, String> {
    let patient: Option<Thing> = db
        .select(("Patient", &data.patient_id))
        .await
        .map_err(|e| e.to_string())?;
    let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

    let user_owner: Option<Thing> = db
        .select(("User", &data.user_owner))
        .await
        .map_err(|e| e.to_string())?;
    let user_owner = user_owner.ok_or_else(|| "Patient not found".to_string())?;

    let updated_note = PatientNoteRecord {
        patient: patient,
        symptoms: data.symptoms,
        diagnosis: data.diagnosis,
        treatment: data.treatment,
        severity: data.severity,
        is_urgent: data.is_urgent,
        user_owner: user_owner,
    };

    let updated: Option<PatientNoteResponse> = db
        .update(("PatientNote", id))
        .merge(updated_note)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated)
}

pub async fn delete_patient_note_service(
    db: &Surreal<Db>,
    id: String,
) -> Result<Option<PatientNoteResponse>, String> {
    let deleted: Option<PatientNoteResponse> = db
        .delete(("PatientNote", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}
