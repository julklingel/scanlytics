use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRecord {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub role: String,
    pub specialization: String,
    pub patients: Option<Vec<Thing>>,
    pub patient_notes: Option<Vec<Thing>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub role: String,
    pub specialization: String,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub role: String,
    pub specialization: String,
    pub patients: Option<Vec<Thing>>,
    pub patient_notes: Option<Vec<Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}