use super::models;
use reqwest::Client as HttpClient;
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
        .json(&login_request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    println!("Response status: {}", response.status());

    if response.status().is_success() {
        let login_response: models::LoginResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        println!("Login successful: {:?}", login_response);
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