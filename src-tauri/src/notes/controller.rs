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
    let mut data: Vec<models::PatientNoteResponse> = services::create_patient_note_service(&db, patient_note_request).await?;
    
    if data.is_empty() {
        return Err("No record created".to_string());
    }
    
    let response = data.pop().unwrap();
    let response = models::PatientNoteResponse {
        id: response.id,
        patient: response.patient,
        symptoms: response.symptoms,
        diagnosis: response.diagnosis,
        treatment: response.treatment,
        severity: response.severity,
        is_urgent: response.is_urgent,
        user_owner: response.user_owner,
        created_at: response.created_at,
        updated_at: response.updated_at,
    };
   
    Ok(response)
}


#[tauri::command]
pub async fn get_patient_notes(
    db: State<'_, RwLock<Surreal<Client>>>,
) -> Result<Vec<models::PatientNoteWithPatientResponse>, String> {
    println!("Watson we arrived in the notes controller");
    let db = db.write().await;
    let response:Vec<models::PatientNoteWithPatientResponse> = services::get_patient_notes_service(&db).await?
        .into_iter()
        .map(|record| models::PatientNoteWithPatientResponse {
            id: record.id,
            created_at: record.created_at,
            diagnosis: record.diagnosis,
            is_urgent: record.is_urgent,
            patient: models::PatientInfo {
                id: record.patient.id,
                name: record.patient.name,
            },
            severity: record.severity,
            symptoms: record.symptoms,
            treatment: record.treatment,
            updated_at: record.updated_at,
            user_owner: models::UserInfo {
                id: record.user_owner.id,
                name: record.user_owner.name,
            },
        })
        .collect();
    println!("Response: {:?}", response);
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
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
) -> Result<models::PatientNoteResponse, String> {
    let db = db.write().await;
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
            updated_at: record.updated_at
        };
       
        Ok(response)
    } else {
        Err("No record deleted".to_string())
    }
}