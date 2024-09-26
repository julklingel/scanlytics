use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetPasswordRequest {
    pub username: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
}
