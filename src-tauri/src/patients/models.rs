

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientRequest {
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub primary_doctor: Thing,  
    pub notes: Vec<Thing>,  
    pub created_at: Datetime,
    pub updated_at: Datetime,
}






#[derive(Debug, Serialize, Deserialize)]
pub struct PatientRecord {
    pub id: Thing,
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub primary_doctor: Thing,  
    pub notes: Vec<Thing>, 
    pub created_at: Datetime,
    pub updated_at: Datetime,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientResponse {
    pub id: Thing,
    pub name: String,
    pub date_of_birth: Datetime,
    pub gender: String,
    pub contact_number: String,
    pub address: String,
    pub primary_doctor: Thing,  
    pub notes: Vec<Thing>,  
    pub created_at: Datetime,
    pub updated_at: Datetime,
}