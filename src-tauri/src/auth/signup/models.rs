use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SignupError {
    ParseError(String),
    ValidationError(String),
    PasswordMismatch,
    WeakPassword(String),
    NetworkError(String),
    DatabaseError(String),
    ServerError(String),
}

impl std::fmt::Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignupError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SignupError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            SignupError::PasswordMismatch => write!(f, "Passwords do not match"),
            SignupError::WeakPassword(msg) => write!(f, "Password too weak: {}", msg),
            SignupError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            SignupError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            SignupError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupRequest {
    pub full_name: String,
    pub user_email: String,
    pub password: String,
    pub confirm_password: String,
}

impl SignupRequest {
    pub fn validate(&self) -> Result<(), SignupError> {
        if self.password != self.confirm_password {
            return Err(SignupError::PasswordMismatch);
        }
        
        if self.full_name.trim().is_empty() {
            return Err(SignupError::ValidationError("Full name cannot be empty".into()));
        }
        
        if self.user_email.trim().is_empty() {
            return Err(SignupError::ValidationError("Email cannot be empty".into()));
        }
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupServerRequest {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_role: String,
}

impl From<SignupRequest> for SignupServerRequest {
    fn from(req: SignupRequest) -> Self {
        Self {
            user_name: req.full_name,
            user_email: req.user_email,
            user_password: req.password,
            user_role: "user".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupResponse {
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerResponse {
    pub message: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
}