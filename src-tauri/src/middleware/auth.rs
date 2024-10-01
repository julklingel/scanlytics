use keyring::Entry;
use reqwest::Client;
use serde_json::json;
use super::models;

pub async fn validate_token(username: &str) -> Result<(), String> {
 
    let username = username.trim(); 
    
    let entry = Entry::new("com.scanlytics.dev", username)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
    
    
    let stored_token = entry
        .get_password()
        .map_err(|e| format!("Failed to retrieve stored token: {}", e))?;


    let client = Client::new();
    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/validate")
        .json(&json!({
            "token": stored_token
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
