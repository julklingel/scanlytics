
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ PatientRequest, PatientRecord  };



pub async fn create_patient_service(db: &Surreal<Client>, data: PatientRequest) -> Result<Vec<PatientRecord>, String> {
    let created: Vec<PatientRecord> = db
        .create("Patient")
        .content(data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(created)
}



pub async fn get_patient_service(db: &Surreal<Client>) -> Result<Vec<PatientRecord>, String> {
    let records: Vec<PatientRecord> = db
        .select("Patient")
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}


pub async fn update_patient_service(db: &Surreal<Client>, id: String, data: PatientRequest) -> Result<Option<PatientRecord>, String> {
    let updated: Option<PatientRecord> = db
        .update(("Patient", id))
        .merge(data)
        .await
        .map_err(|e| e.to_string())?;
   
    Ok(updated)
}


pub async fn delete_patient_service(db: &Surreal<Client>, id: String) -> Result<Option<PatientRecord>, String> {
    let deleted: Option<PatientRecord> = db
        .delete(("Patient", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}