use serde::{Deserialize, Serialize};
use std::fmt;



#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    ParseError(String),
    NetworkError(String),
    AuthenticationError(String),
    KeyringError(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AuthError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AuthError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AuthError::KeyringError(msg) => write!(f, "Keyring error: {}", msg),
        }
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(err: serde_json::Error) -> Self {
        AuthError::ParseError(err.to_string())
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(err: reqwest::Error) -> Self {
        AuthError::NetworkError(err.to_string())
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
            data: None,
        }
    }
}
