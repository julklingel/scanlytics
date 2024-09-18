use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupResponse {
    pub message: String,
    pub user_id: String,
}

