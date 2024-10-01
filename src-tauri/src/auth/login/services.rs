use super::models;
use keyring::Entry;
use reqwest::Client as HttpClient;
use serde_json::Value;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn login_service(
    _db: &Surreal<Client>,
    login_data: String,
) -> Result<models::LoginResponse, String> {
    let login_request: models::LoginRequest = serde_json::from_str(&login_data)
        .map_err(|e| format!("Failed to parse login request: {}", e))?;
    let client = HttpClient::new();

    let login_record = models::LoginRecord {
        user_email: login_request.username.clone(),
        user_password: login_request.password.clone(),
    };

    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/login")
        .json(&login_record)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    println!("Response status: {}", response.status());

    if response.status().is_success() {
        let login_response: models::LoginResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

        if login_response.token_type.to_lowercase() != "bearer" {
            return Err("Token type is not bearer".into());
        }

        let username = login_record.user_email.trim(); 
    
        let entry = Entry::new("com.scanlytics.dev", username)
            .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

        entry
            .set_password(&login_response.access_token)
            .map_err(|e| format!("Failed to store token: {}", e))?;

        let stored_token = entry
            .get_password()
            .map_err(|e| format!("Failed to retrieve stored token: {}", e))?;


        if stored_token != login_response.access_token {
            return Err(
                "Token verification failed: stored token does not match the received token".into(),
            );
        }

        Ok(login_response)
    } else {
        let error_response: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse error response: {}", e))?;

        Err(format!(
            "Login failed: {}",
            error_response["detail"].as_str().unwrap_or("Unknown error")
        ))
    }
}

pub async fn reset_password_service(
    reset_data: String,
) -> Result<models::ResetPasswordResponse, String> {
    let reset_request: models::ResetPasswordRequest = serde_json::from_str(&reset_data)
        .map_err(|e| format!("Failed to parse reset password request: {}", e))?;

    println!("Reset password request: {:?}", reset_request);

    Ok(models::ResetPasswordResponse {
        success: true,
        message: "Password reset successfully".to_string(),
    })
}
