use super::models::{TokenError, TokenResponse};
#[cfg(not(test))]
use keyring::Entry;
use reqwest::Client;
use serde_json::Value;

#[cfg(not(test))]
const SERVICE_NAME: &str = "com.scanlytics.dev";
const API_URL: &str = "https://scanlyticsbe.fly.dev/auth/validate";

#[cfg(test)]
thread_local! {
    pub static TEST_API_URL: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

/// Validates and potentially renews an authentication token.
///
/// # Arguments
///
/// * `user_email` - Email address associated with the token
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(())` - Token is valid and renewed if necessary
/// * `Err(TokenError)` - Validation error details
///
/// # Errors
///
/// This function can return several types of errors:
/// * `TokenError::KeyringAccess` - Failed to access stored token
/// * `TokenError::ServerError` - Backend communication failed
/// * `TokenError::ValidationError` - Token validation failed
/// * `TokenError::ParseError` - Response parsing failed

pub async fn validate_token_service(user_email: &str) -> Result<(), TokenError> {
    let user_email = user_email.trim();
    let stored_token = get_stored_token(user_email)?;
    let token_response = validate_token_with_api(&stored_token).await?;
    store_new_token(user_email, &token_response.access_token)?;
    Ok(())
}



/// Retrieves stored token from system keyring.
///
/// # Arguments
///
/// * `user_email` - Email address associated with the token
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(String)` - Retrieved token
/// * `Err(TokenError)` - Access error details

#[cfg_attr(test, allow(unused_variables))]
fn get_stored_token(user_email: &str) -> Result<String, TokenError> {
    #[cfg(test)]
    return mock_get_stored_token(user_email);

    #[cfg(not(test))]
    {
        let entry = Entry::new(SERVICE_NAME, user_email).map_err(|e| {
            TokenError::KeyringAccess(format!("Failed to create keyring entry: {}", e))
        })?;

        entry.get_password().map_err(|e| {
            TokenError::KeyringAccess(format!("Failed to retrieve stored token: {}", e))
        })
    }
}

/// Validates token with backend server.
///
/// # Arguments
///
/// * `token` - Token to validate
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(TokenResponse)` - Valid token response
/// * `Err(TokenError)` - Validation error details


async fn validate_token_with_api(token: &str) -> Result<TokenResponse, TokenError> {
    let client = Client::new();

    let url = {
        #[cfg(test)]
        {
            TEST_API_URL.with(|test_url| {
                test_url
                    .borrow()
                    .clone()
                    .unwrap_or_else(|| API_URL.to_string())
            })
        }
        #[cfg(not(test))]
        {
            API_URL.to_string()
        }
    };

    println!("Using URL: {}", url);

    let response = client
        .post(url)
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

    let response_array: Vec<Value> = response
        .json()
        .await
        .map_err(|e| TokenError::ParseError(e.to_string()))?;

    if response_array.len() < 2 {
        return Err(TokenError::ValidationError(
            "Incomplete server response".to_string(),
        ));
    }

    let token_data = &response_array[1];

    let token_response = TokenResponse {
        access_token: token_data["access_token"]
            .as_str()
            .ok_or_else(|| TokenError::ParseError("Missing access token".to_string()))?
            .to_string(),
        token_type: token_data["token_type"]
            .as_str()
            .ok_or_else(|| TokenError::ParseError("Missing token type".to_string()))?
            .to_string(),
    };

    token_response.validate()?;
    Ok(token_response)
}

/// Stores new token in system keyring.
///
/// # Arguments
///
/// * `user_email` - Email address associated with the token
/// * `new_token` - New token to store
///
/// # Returns
///
/// Returns a `Result` indicating success or failure


#[cfg_attr(test, allow(unused_variables))]
fn store_new_token(user_email: &str, new_token: &str) -> Result<(), TokenError> {
    #[cfg(test)]
    return mock_store_new_token(user_email, new_token);

    #[cfg(not(test))]
    {
        let entry = Entry::new(SERVICE_NAME, user_email).map_err(|e| {
            TokenError::KeyringAccess(format!("Failed to create keyring entry: {}", e))
        })?;

        entry
            .set_password(new_token)
            .map_err(|e| TokenError::KeyringStore(format!("Failed to store token: {}", e)))
    }
}

/// Authentication middleware for protected routes.
///
/// # Arguments
///
/// * `user_email` - Email address for authentication
/// * `f` - Protected async function to execute
///
/// # Returns
///
/// Returns the result of the protected function if authentication succeeds
///
/// # Example
///
/// ```rust,no_run
/// async fn protected_route() -> Result<String, String> {
///     Ok("Protected data".to_string())
/// }
///
/// auth_middleware("user@example.com", protected_route).await

pub async fn auth_middleware<F, Fut, R>(user_email: &str, f: F) -> Result<R, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<R, String>>,
{
    validate_token_service(user_email)
        .await
        .map_err(|e| e.to_string())?;
    f().await
}

#[cfg(test)]
mod mock_keyring {
    use super::*;
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static! {
        pub static ref MOCK_KEYRING: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }

    pub fn mock_get_stored_token(user_email: &str) -> Result<String, TokenError> {
        let store = MOCK_KEYRING.lock().unwrap();
        store
            .get(user_email)
            .cloned()
            .ok_or_else(|| TokenError::KeyringAccess("Token not found".into()))
    }

    pub fn mock_store_new_token(user_email: &str, new_token: &str) -> Result<(), TokenError> {
        let mut store = MOCK_KEYRING.lock().unwrap();
        store.insert(user_email.to_string(), new_token.to_string());
        Ok(())
    }
}

#[cfg(test)]
pub use mock_keyring::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_mock_token_response() -> Value {
        json!([
            "message",
            {
                "access_token": "new_valid_token_123",
                "token_type": "Bearer"
            }
        ])
    }

    struct TestContext {
        mock_server: MockServer,
    }

    impl TestContext {
        async fn new() -> Self {
            let mock_server = MockServer::start().await;
            let api_url = mock_server.uri() + "/auth/validate";

            TEST_API_URL.with(|url| {
                *url.borrow_mut() = Some(api_url);
            });

            Self { mock_server }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            TEST_API_URL.with(|url| {
                *url.borrow_mut() = None;
            });

            MOCK_KEYRING.lock().unwrap().clear();
        }
    }

    #[tokio::test]
    async fn test_successful_token_validation() {
        let ctx = TestContext::new().await;
        let mock_response = create_mock_token_response();
        let test_token = "test_token";

        Mock::given(method("POST"))
            .and(path("/auth/validate"))
            .and(header("Authorization", format!("Bearer {}", test_token)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .expect(1)
            .mount(&ctx.mock_server)
            .await;

        let result = validate_token_with_api(test_token).await;
        assert!(result.is_ok());
        if let Ok(response) = result {
            assert_eq!(response.access_token, "new_valid_token_123");
            assert_eq!(response.token_type, "Bearer");
        }
    }

    #[tokio::test]
    async fn test_invalid_token_response() {
        let ctx = TestContext::new().await;
        let test_token = "test_token";

        let mock_response = json!(["message"]);

        Mock::given(method("POST"))
            .and(path("/auth/validate"))
            .and(header("Authorization", format!("Bearer {}", test_token)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .expect(1)
            .mount(&ctx.mock_server)
            .await;

        let result = validate_token_with_api(test_token).await;
        assert!(matches!(result, Err(TokenError::ValidationError(_))));
    }

    #[tokio::test]
    async fn test_server_error_response() {
        let ctx = TestContext::new().await;
        let test_token = "test_token";

        Mock::given(method("POST"))
            .and(path("/auth/validate"))
            .and(header("Authorization", format!("Bearer {}", test_token)))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&ctx.mock_server)
            .await;

        let result = validate_token_with_api(test_token).await;
        assert!(matches!(result, Err(TokenError::ValidationError(_))));
    }

    #[tokio::test]
    async fn test_malformed_response() {
        let ctx = TestContext::new().await;
        let test_token = "test_token";

        Mock::given(method("POST"))
            .and(path("/auth/validate"))
            .and(header("Authorization", format!("Bearer {}", test_token)))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .expect(1)
            .mount(&ctx.mock_server)
            .await;

        let result = validate_token_with_api(test_token).await;
        assert!(matches!(result, Err(TokenError::ParseError(_))));
    }

    #[tokio::test]
    async fn test_auth_middleware() {
        let ctx = TestContext::new().await;
        let mock_response = create_mock_token_response();
        let test_token = "test_token";
        let test_email = "test@example.com";

        mock_store_new_token(test_email, test_token).expect("Failed to store test token");

        Mock::given(method("POST"))
            .and(path("/auth/validate"))
            .and(header("Authorization", format!("Bearer {}", test_token)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .expect(1)
            .mount(&ctx.mock_server)
            .await;

        let result = auth_middleware(test_email, || async { Ok("Success".to_string()) }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
    }

    #[tokio::test]
    async fn test_auth_middleware_with_invalid_token() {
        let _ctx = TestContext::new().await;
        let test_email = "test@example.com";

        let result = auth_middleware(test_email, || async { Ok("Success".to_string()) }).await;

        assert!(result.is_err());
    }
    #[test]
    fn test_keyring_store_error() {
        let error = TokenError::KeyringStore("test error".to_string());
        assert!(matches!(error, TokenError::KeyringStore(_)));
    }

    #[test]
    fn test_keyring_operations() {
        let test_email = "test@example.com";
        let test_token = "test_token";

        let store_result = store_new_token(test_email, test_token);
        assert!(store_result.is_ok());

        let get_result = get_stored_token(test_email);
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), test_token);

        let invalid_result = get_stored_token("nonexistent@example.com");
        assert!(matches!(invalid_result, Err(TokenError::KeyringAccess(_))));
    }
}
