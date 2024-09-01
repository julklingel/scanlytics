use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportRequest {
    pub patient_id: String,
    pub user_owner: String,
    pub report_text: String,
    pub files: Vec<String>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportRecord {
    pub patient: Thing,
    pub user_owner: Thing,
    pub report_text: String,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportResponse {
    pub id: Thing,
    pub patient: Thing,
    pub user_owner: Thing,
    pub report_text: String,
    pub files: Vec<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatientInfo {
    pub id: Thing,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: Thing,
    pub name: String,
}