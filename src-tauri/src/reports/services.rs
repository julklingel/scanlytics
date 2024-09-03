// use std::fmt::Result;

// use super::models::{ ReportResponse, ReportRequest, PatientResponse, UserResponse
// };
// use surrealdb::engine::remote::ws::Client;

// use surrealdb::sql::Thing;
// use surrealdb::Error as SurrealError;
// use surrealdb::Surreal;
// use tauri::State;

// pub async fn create_report_service(
//     db: &Surreal<Client>,
//     data: ReportRequest,
// ) -> Result<Vec<ReportResponse>, String>  {
//     let patient: Option<PatientResponse> = db
//         .select(("Patient", &data.patient_id))
//         .await
//         .map_err(|e| e.to_string())?;
//     let patient = patient.ok_or_else(|| "Patient not found".to_string())?;

//     let user_owner: Option<UserResponse> = db
//         .select(("User", &data.user_owner))
//         .await
//         .map_err(|e| e.to_string())?;
//     let user_owner = user_owner.ok_or_else(|| "User not found".to_string())?;
// }
