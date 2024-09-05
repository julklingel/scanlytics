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
    pub extension: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportRecord {
    pub patient: Thing,
    pub user_owner: Thing,
    pub report_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResponse {
    pub id: Thing,
    pub name: String,
    pub path: String,
    pub patient: surrealdb::sql::Thing,
    pub user: surrealdb::sql::Thing,
    pub file_type: String,
    pub modal_type: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRequest {
    pub name: String,
    pub path: String,
    pub patient: Thing,
    pub user: Thing,
    pub file_type: String,
    pub modal_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportResponse {
    pub id: Thing,
    pub patient: PatientInfo,
    pub user_owner: UserInfo,
    pub report_text: String,
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
