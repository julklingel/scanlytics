

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};




#[derive(Debug, Serialize, Deserialize)]
pub struct PatientNoteRequest {
    pub patient_id: String,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub follow_up_date: Option<String>,
    pub severity: String,
    pub is_urgent: bool,
    pub department: String,
    pub attending_doctor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientNoteRecord {

    pub patient: Thing,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub is_urgent: bool,
    pub department: String,
    pub attending_doctor: Thing,
    pub severity: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientNoteResponse {
    pub id: Thing,
    pub patient: Thing,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub is_urgent: bool,
    pub department: String,
    pub attending_doctor: Thing,
    pub severity: String,
    pub created_at: Datetime,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoctorRecord {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub role: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientRecord {
    pub id: Thing,
    pub name: String,
}