use super::models::{PatientRecord, PatientRequest, PatientResponse, UserResponse};

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn create_patient_service(
    db: &Surreal<Client>,
    data: PatientRequest,
) -> Result<Vec<PatientResponse>, String> {
    let doctor: Option<UserResponse> = db
        .select(("User", &data.primary_doctor))
        .await
        .map_err(|e| e.to_string())?;
    let doctor = doctor.ok_or_else(|| "Doctor not found".to_string())?;

    let patient_record = PatientRecord {
        name: data.name,
        date_of_birth: data.date_of_birth,
        gender: data.gender,
        contact_number: data.contact_number,
        address: data.address,
        notes: data.notes,
        reports: data.reports,
        images: data.images,
    };

    let created: Vec<PatientResponse> = db
        .create("Patient")
        .content(patient_record)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(patient) = created.first() {
        db.query("RELATE $patient -> Treated_By -> $doctor")
            .bind(("patient", &patient.id))
            .bind(("doctor", &doctor.id))
            .await
            .map_err(|e| e.to_string())?;
        println!("Patient created: {:?}", patient);
    }

    Ok(created)
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
