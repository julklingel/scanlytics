use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupRequest {
    pub full_name: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupResponse {
    pub message: String,
   
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupRecord {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_role: String,
}