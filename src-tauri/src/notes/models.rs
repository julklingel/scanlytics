

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};


#[derive(Debug, Serialize, Deserialize)]
pub struct PatientNoteRequest {
    pub patient_id: String,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub severity: String,
    pub is_urgent: bool,
    pub user_owner: Thing,
   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientNoteRecord {
    pub patient: Thing,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub severity: String,
    pub is_urgent: bool,
    pub user_owner: Thing,
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientNoteResponse {
    pub id: Thing,
    pub patient: Thing,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub severity: String,
    pub is_urgent: bool,
    pub user_owner: Thing, 
    pub created_at: Datetime,
    pub updated_at: Datetime, 
    
}