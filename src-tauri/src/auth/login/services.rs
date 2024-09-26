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

    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/login")
        .form(&[
            ("username", &login_request.username),
            ("password", &login_request.password),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    println!("Response status: {}", response.status());

    if response.status().is_success() {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {}", e))?;

        let v: Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

        let access_token = v["access_token"]
            .as_str()
            .ok_or("access_token not found or not a string")?;

        let token_type = v["token_type"]
            .as_str()
            .ok_or("token_type not found or not a string")?;

        if token_type.to_lowercase() != "bearer" {
            return Err("Token type is not bearer".into());
        }

        let login_response = models::LoginResponse {
            access_token: access_token.to_string(),
            token_type: token_type.to_string(),
        };

        let entry = Entry::new("Scanlytics", &login_request.username)
            .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
        entry
            .set_password(&login_response.access_token)
            .map_err(|e| format!("Failed to store token: {}", e))?;

        Ok(login_response)
    } else {
        Err(format!("Login failed: {}", response.status()))
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
