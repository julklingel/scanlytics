use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};





#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationRequest {
    pub name: String,
    pub address: String,
    pub email: String,
    pub users: Option<Vec<Thing>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationRecord {
    pub id: Thing,
    pub name: String,
    pub address: String,
    pub email: String,
    pub users: Option<Vec<Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationResponse {
    pub id: Thing,
    pub name: String,
    pub address: String,
    pub email: String,
    pub users: Option<Vec<Thing>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}