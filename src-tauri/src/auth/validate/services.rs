use reqwest::Client;
use keyring::Entry;
use super::models;



pub async fn validate_token_service(user_email: &str) -> Result<(), String> {
    let user_email = user_email.trim();
    let entry = Entry::new("com.scanlytics.dev", user_email)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    let stored_token = entry
        .get_password()
        .map_err(|e| format!("Failed to retrieve stored token: {}", e))?;

    let client = Client::new();
    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/validate")
        .header("Authorization", format!("Bearer {}", stored_token))
        .send()
        .await
        .map_err(|e| format!("Failed to send validation request: {}", e))?;

    if response.status().is_success() {
        let token_response: models::TokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

        if token_response.token_type.to_lowercase() != "bearer" {
            return Err("Token type is not bearer".into());
        }

        let entry = Entry::new("com.scanlytics.dev", user_email)
            .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

        entry
            .set_password(&token_response.access_token)
            .map_err(|e| format!("Failed to store token: {}", e))?;

        Ok(())
    } else {
        Err(format!("Token validation failed: {}", response.status()))
    }
}


pub async fn auth_middleware<F, Fut, R>(
    user_email: &str,
    f: F,
) -> Result<R, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<R, String>>,
{
    validate_token_service(user_email).await?;
    f().await
}
