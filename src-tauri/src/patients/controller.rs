use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn create_patient(
    db: State<'_, RwLock<Surreal<Client>>>,
    patient_note_request: String,
) -> Result<models::PatientResponse, String> {
    let patient_note_request: models::PatientRequest = serde_json::from_str(&patient_note_request)
        .map_err(|e| format!("Failed to parse patient note request: {}", e))?;
    let db = db.write().await;
    let mut records = services::create_patient_service(&db, patient_note_request).await?;
    if records.is_empty() {
        return Err("No record created".to_string());
    }
    let record = records.pop().unwrap();
    
    let response = models::PatientResponse {
        id: record.id,
        name: record.name,
        patient_id: record.patient_id,
        date_of_birth: record.date_of_birth,
        gender: record.gender,
        contact_number: record.contact_number,
        address: record.address,
        primary_doctor: record.primary_doctor,
        notes: record.notes,
        created_at: record.created_at,	
        updated_at: record.updated_at,

    };
    
    Ok(response)
}

#[tauri::command]
pub async fn get_patients(
    db: State<'_, RwLock<Surreal<Client>>>,
) -> Result<Vec<models::PatientResponse>, String> {
    let db = db.write().await;
    let records = services::get_patient_service(&db).await?;
    let response = records
        .iter()
        .map(|record| models::PatientResponse {
            id: record.id.clone(),
            name: record.name.clone(),
            patient_id: record.patient_id.clone(),
            date_of_birth: record.date_of_birth.clone(),
            gender: record.gender.clone(),
            contact_number: record.contact_number.clone(),
            address: record.address.clone(),
            primary_doctor: record.primary_doctor.clone(),
            notes: record.notes.clone(),
            created_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        })
        .collect();
    Ok(response)
}

#[tauri::command]
pub async fn update_patient(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    println!("update_patient: id: {}, patient_request: {}", id, patient_request);
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;
    
    let db = db.write().await;
    let updated_record = services::update_patient_service(&db, id, patient_request).await?;
    
    if let Some(record) = updated_record {
        let response = models::PatientResponse {
            id: record.id,
            name: record.name,
            patient_id: record.patient_id,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            primary_doctor: record.primary_doctor,
            notes: record.notes,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };
        
        Ok(response)
    } else {
        Err("No record updated".to_string())
    }
}

#[tauri::command]
pub async fn delete_patient(
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
) -> Result<models::PatientResponse, String> {
    let db = db.write().await;
    let deleted_record = services::delete_patient_service(&db, id).await?;
    
    if let Some(record) = deleted_record {
        let response = models::PatientResponse {
            id: record.id,
            name: record.name,
            patient_id: record.patient_id,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            primary_doctor: record.primary_doctor,
            notes: record.notes,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };
        
        Ok(response)
    } else {
        Err("No record deleted".to_string())
    }
}