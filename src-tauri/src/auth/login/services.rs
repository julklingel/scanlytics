use super::models::{ApiResponse, AuthError, LoginRequest, LoginResponse, ResetPasswordRequest};
use keyring::Entry;
use reqwest::Client as HttpClient;

pub async fn login_service(login_data: String) -> Result<ApiResponse<LoginResponse>, AuthError> {
    let login_request: LoginRequest = serde_json::from_str(&login_data)?;
    let client = HttpClient::new();

    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/login")
        .json(&login_request)
        .send()
        .await?;

    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await?;

        if login_response.token_type.to_lowercase() != "bearer" {
            return Ok(ApiResponse::error("Invalid token type".to_string()));
        }

        store_token(&login_request.email, &login_response.access_token)?;
        Ok(ApiResponse::success(login_response))
    } else {
        let error_response: serde_json::Value = response.json().await?;
        Ok(ApiResponse::error(
            error_response["detail"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string(),
        ))
    }
}

pub async fn reset_password_service(
    reset_data: String,
) -> Result<ApiResponse<ResetPasswordRequest>, AuthError> {
    let reset_request: ResetPasswordRequest = serde_json::from_str(&reset_data)?;
    
    Ok(ApiResponse::success(reset_request))
}

fn store_token(username: &str, token: &str) -> Result<(), AuthError> {
    let entry = Entry::new("com.scanlytics.dev", username.trim()).map_err(|e| {
        AuthError::KeyringError(format!("Failed to create keyring entry: {}", e))
    })?;

    entry.set_password(token).map_err(|e| {
        AuthError::KeyringError(format!("Failed to store token: {}", e))
    })?;

    Ok(())
}
