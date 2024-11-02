use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub role: String,
    pub organization: Option<Thing>,
    pub patients: Option<Vec<surrealdb::sql::Thing>>,
    pub patient_notes: Option<Vec<surrealdb::sql::Thing>>,
    pub statements: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRecord {
    pub name: String,
    pub email: String,
    pub role: String,
    pub organization: Option<Thing>,
    pub patients: Option<Vec<surrealdb::sql::Thing>>,
    pub patient_notes: Option<Vec<surrealdb::sql::Thing>>,
    pub statements: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub role: String,
    pub organization: Option<Thing>,
    pub patients: Option<Vec<surrealdb::sql::Thing>>,
    pub patient_notes: Option<Vec<surrealdb::sql::Thing>>,
    pub statements: Option<Vec<surrealdb::sql::Thing>>,
    pub images: Option<Vec<surrealdb::sql::Thing>>,
    pub reports: Option<Vec<surrealdb::sql::Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
