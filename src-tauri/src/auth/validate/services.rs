use reqwest::Client;
use keyring::Entry;
use super::models::{TokenError, TokenResponse};

const SERVICE_NAME: &str = "com.scanlytics.dev";
const API_URL: &str = "https://scanlyticsbe.fly.dev/auth/validate";

pub async fn validate_token_service(user_email: &str) -> Result<(), TokenError> {
    let user_email = user_email.trim();
    let stored_token = get_stored_token(user_email)?;
    let token_response = validate_token_with_api(&stored_token).await?;
    store_new_token(user_email, &token_response.access_token)?;
    Ok(())
}

fn get_stored_token(user_email: &str) -> Result<String, TokenError> {
    let entry = Entry::new(SERVICE_NAME, user_email).map_err(|e| {
        TokenError::KeyringAccess(format!("Failed to create keyring entry: {}", e))
    })?;

    entry.get_password().map_err(|e| {
        TokenError::KeyringAccess(format!("Failed to retrieve stored token: {}", e))
    })
}

async fn validate_token_with_api(token: &str) -> Result<TokenResponse, TokenError> {
    let client = Client::new();
    let response = client
        .post(API_URL)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| TokenError::ServerError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(TokenError::ValidationError(format!(
            "Token validation failed: {}",
            response.status()
        )));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| TokenError::ParseError(e.to_string()))?;

    token_response.validate()?;
    Ok(token_response)
}

fn store_new_token(user_email: &str, new_token: &str) -> Result<(), TokenError> {
    let entry = Entry::new(SERVICE_NAME, user_email).map_err(|e| {
        TokenError::KeyringAccess(format!("Failed to create keyring entry: {}", e))
    })?;

    entry.set_password(new_token).map_err(|e| {
        TokenError::KeyringStore(format!("Failed to store token: {}", e))
    })
}


pub async fn auth_middleware<F, Fut, R>(
    user_email: &str,
    f: F,
) -> Result<R, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<R, String>>,
{
    validate_token_service(user_email)
        .await
        .map_err(|e| e.to_string())?;
    f().await
}
