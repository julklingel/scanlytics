
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use super::models::{ PatientNoteRecord, PatientNoteRequest, PatientNoteResponse}; 



// pub async fn create_patient_note_service(db: &Surreal<Client>, data: PatientNoteRequest) -> Result<Vec<PatientNoteResponse>, String> {
//     let doctor: Option<DoctorRecord> = db
//     .select(("User", &data.attending_doctor))
//     .await
//     .map_err(|e| e.to_string())?;
//     let doctor = doctor.ok_or_else(|| "Doctor not found".to_string())?;

//     let patient: Option<PatientRecord> = db
//     .select(("Patient", &data.patient_id))
//     .await
//     .map_err(|e| e.to_string())?;
//     let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

//     let data = PatientNoteRecord {
//         patient: patient.id,
//         symptoms: data.symptoms,
//         diagnosis: data.diagnosis,
//         treatment: data.treatment,
//         is_urgent: data.is_urgent,
//         department: data.department,
//         attending_doctor: doctor.id,
//         severity: data.severity,
//     };
    
//     let created: Vec<PatientNoteResponse> = db
//         .insert("PatientNote")
//         .value(data)
//         .await
//         .map_err(|e| e.to_string())?;
//     Ok(created)
// }



// pub async fn get_patient_notes_service(db: &Surreal<Client>) -> Result<Vec<PatientNoteResponse>, String> {
//     let records: Vec<PatientNoteResponse> = db
//         .select("PatientNote")
//         .await
//         .map_err(|e| e.to_string())?;
//     Ok(records)
// }


// pub async fn update_patient_note_service(db: &Surreal<Client>, id: String, data: PatientNoteRequest) -> Result<Option<PatientNoteResponse>, String> {
//     let doctor: Option<DoctorRecord> = db
//     .select(("User", &data.attending_doctor))
//     .await
//     .map_err(|e| e.to_string())?;
//     let doctor = doctor.ok_or_else(|| "Doctor not found".to_string())?;

//     let patient: Option<PatientRecord> = db
//     .select(("Patient", &data.patient_id))
//     .await
//     .map_err(|e| e.to_string())?;
//     let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

//     let data = PatientNoteRecord {
//         patient: patient.id,
//         symptoms: data.symptoms,
//         diagnosis: data.diagnosis,
//         treatment: data.treatment,
//         is_urgent: data.is_urgent,
//         department: data.department,
//         attending_doctor: doctor.id,
//         severity: data.severity,
//     };
    
    
//     let updated: Option<PatientNoteResponse> = db
//         .update(("PatientNote", id))
//         .merge(data)
//         .await
//         .map_err(|e| e.to_string())?;
   
//     Ok(updated)
// }


// pub async fn delete_patient_note_service(db: &Surreal<Client>, id: String) -> Result<Option<PatientNoteResponse>, String> {
//     let deleted: Option<PatientNoteResponse> = db
//         .delete(("PatientNote", id))
//         .await
//         .map_err(|e| e.to_string())?;
//     Ok(deleted)
// }