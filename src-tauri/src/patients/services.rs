use super::models::{ PatientRecord, PatientRequest, PatientResponse};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;


pub async fn create_patient_service(
    db: &Surreal<Client>,
    data: PatientRequest,
) -> Result<Vec<PatientResponse>, String> {

    // Get treated by data entry


    let patient_record = PatientRecord {
        name: data.name,
        date_of_birth: data.date_of_birth,
        gender: data.gender,
        contact_number: data.contact_number,
        address: data.address,
        notes: data.notes,
        images: data.images,
        reports: data.reports,
    };


    let response: Vec<PatientResponse> = db
        .create("Patient")
        .content(patient_record)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

pub async fn get_patient_service(db: &Surreal<Client>) -> Result<Vec<PatientResponse>, String> {
    let records: Vec<PatientResponse> = db.select("Patient").await.map_err(|e| e.to_string())?;
    Ok(records)
}

pub async fn update_patient_service(
    db: &Surreal<Client>,
    id: String,
    data: PatientRequest,
) -> Result<Option<PatientResponse>, String> {

    // Get treated by data


    let patient_record = PatientRecord {
        name: data.name,
        date_of_birth: data.date_of_birth,
        gender: data.gender,
        contact_number: data.contact_number,
        address: data.address,
        notes: data.notes,
        images: data.images,
        reports: data.reports,
    };


    let updated: Option<PatientResponse> = db
        .update(("Patient", id))
        .merge(patient_record)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated)
}

pub async fn delete_patient_service(
    db: &Surreal<Client>,
    id: String,
) -> Result<Option<PatientResponse>, String> {
    let deleted: Option<PatientResponse> = db
        .delete(("Patient", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}
