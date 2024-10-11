use super::models;
use super::services;

use crate::db::models::DbConnection;
use tauri::State;

#[tauri::command]
pub async fn create_patient(
    db_connection: State<'_, DbConnection>,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    let db = db_connection.get().lock().await;
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;
    
    let response: models::PatientResponse =
        services::create_patient_service(&db, patient_request).await?;
    Ok(response)
}

#[tauri::command]
pub async fn get_patients(
    db_connection: State<'_, DbConnection>,
) -> Result<Vec<models::PatientResponse>, String> {
    let db = db_connection.get().lock().await;
    let response: Vec<models::PatientResponse> = services::get_patient_service(&db)
        .await?
        .into_iter()
        .map(|record| models::PatientResponse {
            id: record.id,
            name: record.name,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            notes: record.notes,
            reports: record.reports,
            images: record.images,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
        .collect();
    Ok(response)
}

#[tauri::command]
pub async fn update_patient(
    db_connection: State<'_, DbConnection>,
    id: String,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    let db = db_connection.get().lock().await;
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;

   
    let updated_record = services::update_patient_service(&db, id, patient_request).await?;

    if let Some(record) = updated_record {
        let response = models::PatientResponse {
            id: record.id,
            name: record.name,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            notes: record.notes,
            reports: record.reports,
            images: record.images,
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
    db_connection: State<'_, DbConnection>,
    id: String,
) -> Result<models::PatientResponse, String> {
    let db = db_connection.get().lock().await;
    let deleted_record = services::delete_patient_service(&db, id).await?;

    if let Some(record) = deleted_record {
        let response = models::PatientResponse {
            id: record.id,
            name: record.name,
            date_of_birth: record.date_of_birth,
            gender: record.gender,
            contact_number: record.contact_number,
            address: record.address,
            notes: record.notes,
            reports: record.reports,
            images: record.images,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };

        Ok(response)
    } else {
        Err("No record deleted".to_string())
    }
}
