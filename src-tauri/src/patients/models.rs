use serde::{Deserialize, Serialize};
use scanlytics_db::{Thing, Datetime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientRequest {
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub notes: Option<Vec<Thing>>,
    pub reports: Option<Vec<Thing>>,
    pub images: Option<Vec<Thing>>,
    pub primary_doctor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientRecord {
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub notes: Option<Vec<Thing>>,
    pub reports: Option<Vec<Thing>>,
    pub images: Option<Vec<Thing>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientResponse {
    pub id: Thing,
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub notes: Option<Vec<Thing>>,
    pub reports: Option<Vec<Thing>>,
    pub images: Option<Vec<Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub role: String,
    pub organization: Option<Thing>,
    pub patients: Option<Vec<Thing>>,
    pub patient_notes: Option<Vec<Thing>>,
    pub statements: Option<Vec<Thing>>,
    pub images: Option<Vec<Thing>>,
    pub reports: Option<Vec<Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
