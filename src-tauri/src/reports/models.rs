use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    pub patient_id: String,
    pub user_owner: String,
    pub report_text: String,
    pub files: Vec<FileData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileData {
    pub filename: String,
    pub content: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportRecord {
    pub patient: Thing,
    pub user_owner: Thing,
    pub report_text: String,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRecord {
    pub id: String,
    pub name: String,
    pub path: String,
    pub patient: String,
    pub file_type: String,
    pub body_type: String,
    pub modality_type: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRequest {

    pub name: String,
    pub path: String,
    pub patient: String,
    pub file_type: String,
    pub modality_type: String,

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
