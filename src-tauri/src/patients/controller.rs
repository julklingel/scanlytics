use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn create_patient(
    db: State<'_, RwLock<Surreal<Client>>>,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;
    let db = db.write().await;
    let mut data: Vec<models::PatientResponse> = services::create_patient_service(&db, patient_request).await?;
    if data.is_empty() {
        return Err("No record created".to_string());
    }
    let response = data.pop().unwrap();
    let response = models::PatientResponse {
        id: response.id,
        name: response.name,
        date_of_birth: response.date_of_birth,
        gender: response.gender,
        contact_number: response.contact_number,
        address: response.address,
        notes: response.notes,
        reports: response.reports,
        images: response.images,
        created_at: response.created_at,
        updated_at: response.updated_at,
    };
   
    Ok(response)
}


#[tauri::command]
pub async fn get_patients(
    db: State<'_, RwLock<Surreal<Client>>>,
) -> Result<Vec<models::PatientResponse>, String> {

    let db = db.write().await;
    let response:Vec<models::PatientResponse> = services::get_patient_service(&db).await?
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
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
    patient_request: String,
) -> Result<models::PatientResponse, String> {
    
    let patient_request: models::PatientRequest = serde_json::from_str(&patient_request)
        .map_err(|e| format!("Failed to parse patient request: {}", e))?;
    
    let db = db.write().await;
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
    db: State<'_, RwLock<Surreal<Client>>>,
    id: String,
) -> Result<models::PatientResponse, String> {
    println!("delete_patient: id: {}", id);
    let db = db.write().await;
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