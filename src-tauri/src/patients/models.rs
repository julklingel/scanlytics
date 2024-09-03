use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientRequest {
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub notes: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
    pub primary_doctor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientRecord {
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub notes: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientResponse {
    pub id: Thing,
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub notes: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub organization: Thing,
    pub patients: Option<Vec<surrealdb::sql::Thing>>,
    pub patient_notes: Option<Vec<surrealdb::sql::Thing>>,
    pub statements: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
