use keyring::Entry;
use reqwest::Client;
use serde_json::json;
use super::models;

pub async fn validate_token(username: &str) -> Result<(), String> {
    println!("Attempting to validate token for user: {}", username);

    let keyring_entry = Entry::new("Scanlytics", username)
        .map_err(|e| format!("Failed to create keyring entry: {} (username: {})", e, username))?;
    
    println!("Keyring entry created successfully");

    let token = keyring_entry.get_password()
        .map_err(|e| format!("Failed to retrieve token: {} (username: {})", e, username))?;

    println!("Token retrieved successfully");

    let client = Client::new();
    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/validate")
        .json(&json!({
            "token": token
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send validation request: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Token validation failed: {}", response.status()))
    }
}

pub async fn auth_middleware<F, Fut, R>(
    username: &str,
    f: F,
) -> Result<R, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<R, String>>,
{
    validate_token(username).await?;
    f().await
}
