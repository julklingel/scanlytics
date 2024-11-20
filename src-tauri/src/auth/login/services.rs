use super::models::{ApiResponse, AuthError, LoginRequest, LoginResponse};
use keyring::Entry;
use reqwest::Client as HttpClient;
use serde_json::Value;

pub async fn login_service(login_data: String) -> Result<ApiResponse<LoginResponse>, AuthError> {
    let login_request: LoginRequest = serde_json::from_str(&login_data)
        .map_err(|_| AuthError::Parse("Invalid login data".to_string()))?;
    
    let client = HttpClient::new();

    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/login")
        .json(&login_request)
        .send()
        .await
        .map_err(|_| AuthError::Network("Failed to connect to server".to_string()))?;

    if response.status().is_success() {
        let response_array: Vec<Value> = response
            .json()
            .await
            .map_err(|_| AuthError::Parse("Invalid server response".to_string()))?;

        if response_array.len() < 2 {
            return Err(AuthError::Parse("Incomplete server response".to_string()));
        }

        let login_data = &response_array[1];
        
        let login_response = LoginResponse {
            access_token: login_data["access_token"]
                .as_str()
                .ok_or_else(|| AuthError::Parse("Missing access token".to_string()))?
                .to_string(),
            token_type: login_data["token_type"]
                .as_str()
                .ok_or_else(|| AuthError::Parse("Missing token type".to_string()))?
                .to_string(),
        };

        if login_response.token_type.to_lowercase() != "bearer" {
            return Err(AuthError::Authentication("Invalid token type".to_string()));
        }

        store_token(&login_request.user_email, &login_response.access_token)?;
        Ok(ApiResponse::success(login_response))
    } else {
        let error_response: Value = response
            .json()
            .await
            .map_err(|_| AuthError::Parse("Failed to parse error response".to_string()))?;

        println!("{:?}", error_response);

        Err(AuthError::Authentication(
            error_response["detail"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string(),
        ))
    }
}


fn store_token(user_email: &str, token: &str) -> Result<(), AuthError> {
    let entry = Entry::new("com.scanlytics.dev", user_email.trim()).map_err(|e| {
        AuthError::Keyring(format!("Failed to create keyring entry: {}", e))
    })?;

    entry.set_password(token).map_err(|e| {
        AuthError::Keyring(format!("Failed to store token: {}", e))
    })?;

    Ok(())
}