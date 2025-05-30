use serde::{Deserialize, Serialize};
use scanlytics_db::{Thing, Datetime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientNoteRequest {
    pub patient_id: String,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub severity: String,
    pub is_urgent: bool,
    pub user_owner: String,
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
pub struct UserInfo {
    pub id: Thing,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientNoteWithPatientResponse {
    pub id: Thing,
    pub symptoms: String,
    pub diagnosis: String,
    pub treatment: String,
    pub severity: String,
    pub is_urgent: bool,
    pub patient: PatientInfo,
    pub user_owner: UserInfo,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientInfo {
    pub id: Thing,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub role: String,
    pub organization: Option<Thing>,
    pub patients: Option<Vec<Thing>>,
    pub notes: Option<Vec<Thing>>,
    pub statements: Option<Vec<Thing>>,
    pub images: Option<Vec<Thing>>,
    pub reports: Option<Vec<Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
