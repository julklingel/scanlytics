

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;



#[derive(Debug, Serialize, Deserialize)]
pub struct PatientNoteRequest {
    pub patient_name: String,
    pub patient_id: String,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub is_urgent: bool,
    pub department: String,
    pub attending_doctor: String,
    pub severity: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PatientNoteRecord {
    pub id: Thing,
    pub patient_name: String,
    pub patient_id: String,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub is_urgent: bool,
    pub department: String,
    pub attending_doctor: String,
    pub severity: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientNoteResponse {
    pub id: String,
    pub patient_name: String,
    pub patient_id: String,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub is_urgent: bool,
    pub department: String,
    pub attending_doctor: String,
    pub severity: String,
}