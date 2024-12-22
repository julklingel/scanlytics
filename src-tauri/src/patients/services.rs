use super::models::{PatientRecord, PatientRequest, PatientResponse, UserResponse};

use scanlytics_db::{Surreal, Any};

/// Creates a new patient record with associated doctor relationship.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `data` - Patient creation request data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(PatientResponse)` - Successfully created patient record
/// * `Err(String)` - Error message if creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * The specified doctor doesn't exist
/// * Patient creation fails
/// * Relationship creation fails

pub async fn create_patient_service(
    db: &Surreal<Any>,
    data: PatientRequest,
) -> Result<PatientResponse, String> {
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

    let created: PatientResponse = db
        .create("Patient")
        .content(patient_record)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to create patient".to_string())?;

    let patient_id = created.id.clone();
    let doctor_id = doctor.id.clone();

    db.query("RELATE $patient -> Treated_By -> $doctor")
        .bind(("patient", patient_id))
        .bind(("doctor", doctor_id))
        .await
        .map_err(|e| e.to_string())?;

    Ok(created)
}

/// Retrieves all patient records from the database.
///
/// # Arguments
///
/// * `db` - Database connection
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<PatientResponse>)` - List of all patient records
/// * `Err(String)` - Error message if retrieval fails

pub async fn get_patient_service( db: &Surreal<Any>,) -> Result<Vec<PatientResponse>, String> {
    let records: Vec<PatientResponse> = db.select("Patient").await.map_err(|e| e.to_string())?;
    Ok(records)
}
/// Updates an existing patient record.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `id` - Unique identifier of the patient
/// * `data` - Updated patient data
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Option<PatientResponse>)` - Updated patient record if found
/// * `Err(String)` - Error message if update fails

pub async fn update_patient_service(
    db: &Surreal<Any>,
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
/// Deletes a patient record from the database.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `id` - Unique identifier of the patient to delete
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Option<PatientResponse>)` - Deleted patient record if found
/// * `Err(String)` - Error message if deletion fails

pub async fn delete_patient_service(
    db: &Surreal<Any>,
    id: String,
) -> Result<Option<PatientResponse>, String> {
    let deleted: Option<PatientResponse> = db
        .delete(("Patient", id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(deleted)
}
