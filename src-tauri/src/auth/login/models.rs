use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub user_email: String,
    pub user_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetPasswordRequest {
    pub user_email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    Authentication(String),
    Network(String),
    Parse(String),
    Keyring(String),
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::Authentication(detail) => write!(f, "{}", detail),
            AuthError::Network(detail) => write!(f, "{}", detail),
            AuthError::Parse(detail) => write!(f, "{}", detail),
            AuthError::Keyring(detail) => write!(f, "{}", detail),
        }
    }
}
