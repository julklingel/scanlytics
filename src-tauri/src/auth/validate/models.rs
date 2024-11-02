use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TokenError {
    KeyringAccess(String),
    KeyringStore(String),
    ServerError(String),
    ValidationError(String),
    ParseError(String),
    InvalidTokenType,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::KeyringAccess(msg) => write!(f, "Keyring access error: {}", msg),
            TokenError::KeyringStore(msg) => write!(f, "Keyring store error: {}", msg),
            TokenError::ServerError(msg) => write!(f, "Server error: {}", msg),
            TokenError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            TokenError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TokenError::InvalidTokenType => write!(f, "Invalid token type: expected 'bearer'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEmail {
    pub user_email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}

impl TokenResponse {
    pub fn validate(&self) -> Result<(), TokenError> {
        if self.token_type.to_lowercase() != "bearer" {
            return Err(TokenError::InvalidTokenType);
        }
        Ok(())
    }
}