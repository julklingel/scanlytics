use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Username {
    pub username: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}