use super::models::{ApiResponse, AuthError, LoginRequest, LoginResponse};
use keyring::Entry;
use reqwest::Client as HttpClient;
use serde_json::Value;

/// Authenticates a user and manages token storage.
///
/// This service handles:
/// - Parsing login credentials
/// - Making authentication requests to the backend
/// - Processing authentication responses
/// - Storing authentication tokens securely
///
/// # Arguments
///
/// * `login_data` - JSON string containing login credentials
/// * `base_url` - Optional base URL for the authentication server
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(ApiResponse<LoginResponse>)` - Successful authentication with token
/// * `Err(AuthError)` - Various authentication errors
///
/// # Errors
///
/// This function can return several types of errors:
/// * `AuthError::Parse` - Invalid login data or response parsing failure
/// * `AuthError::Network` - Network communication failures
/// * `AuthError::Authentication` - Invalid credentials or server rejection
/// * `AuthError::Keyring` - Token storage failures
pub async fn login_service(login_data: String, base_url: Option<String>) -> Result<ApiResponse<LoginResponse>, AuthError> {
    let login_request: LoginRequest = serde_json::from_str(&login_data)
        .map_err(|_| AuthError::Parse("Invalid login data".to_string()))?;
    
    let client = HttpClient::new();
    
    let url = base_url.unwrap_or_else(|| "https://scanlyticsbe.fly.dev".to_string()) + "/auth/login";

    let response = client
        .post(&url)
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


/// Stores authentication token in the system keyring.
///
/// # Arguments
///
/// * `user_email` - Email address associated with the token
/// * `token` - Authentication token to store
///
/// # Returns
///
/// Returns a `Result` indicating success or failure of token storage
///
/// # Security
///
/// This function uses the system keyring for secure token storage.
fn store_token(user_email: &str, token: &str) -> Result<(), AuthError> {
    let entry = Entry::new("com.scanlytics.dev", user_email.trim()).map_err(|e| {
        AuthError::Keyring(format!("Failed to create keyring entry: {}", e))
    })?;

    entry.set_password(token).map_err(|e| {
        AuthError::Keyring(format!("Failed to store token: {}", e))
    })?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, header, body_json};

    #[tokio::test]
    async fn test_successful_login() {
        let mock_server = MockServer::start().await;
        let mock_url = mock_server.uri(); 
    
        let login_request = json!({
            "user_email": "test@example.com",
            "user_password": "password123"  
        });
    
        let mock_response = json!([
            "success",
            {
                "access_token": "test_token",
                "token_type": "Bearer"
            }
        ]);
    
        Mock::given(method("POST"))
            .and(path("/auth/login"))
            .and(header("content-type", "application/json"))
            .and(body_json(&login_request))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(&mock_response)
                .insert_header("content-type", "application/json"))
            .expect(1)
            .mount(&mock_server)
            .await;
    
        let response = login_service(login_request.to_string(), Some(mock_url)).await;
    
        assert!(response.is_ok());
        if let Ok(api_response) = response {
            assert!(api_response.success);
            assert!(api_response.message.is_none());
            let data = api_response.data.as_ref().expect("Data should be present");
            assert_eq!(data.access_token, "test_token");
            assert_eq!(data.token_type, "Bearer");
        }
    }
    
    #[tokio::test]
    async fn test_failed_login() {
        let mock_server = MockServer::start().await;
        let mock_url = mock_server.uri();
    
        let login_request = json!({
            "user_email": "test@example.com",
            "user_password": "wrong_password" 
        });
    
        let mock_response = json!({
            "detail": "Invalid credentials"
        });
    
        Mock::given(method("POST"))
            .and(path("/auth/login"))
            .and(header("content-type", "application/json"))
            .and(body_json(&login_request))
            .respond_with(ResponseTemplate::new(401)
                .set_body_json(&mock_response)
                .insert_header("content-type", "application/json"))
            .expect(1)
            .mount(&mock_server)
            .await;
    
        let response = login_service(login_request.to_string(), Some(mock_url)).await;
    
        assert!(matches!(response, Err(AuthError::Authentication(_))));
        if let Err(AuthError::Authentication(error_msg)) = response {
            assert_eq!(error_msg, "Invalid credentials");
        }
    }
    
    #[tokio::test]
    async fn test_network_error() {
        let invalid_url = "http://localhost:1";
        let login_data = json!({
            "user_email": "test@example.com",
            "user_password": "password123" 
        });
    
        let response = login_service(login_data.to_string(), Some(invalid_url.to_string())).await;
        assert!(matches!(response, Err(AuthError::Network(_))));
    }

    #[test]
    fn test_invalid_login_data() {
        let invalid_login_data = r#"{
            "invalid": "data"
        }"#;

        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(login_service(invalid_login_data.to_string(), None));
        assert!(matches!(response, Err(AuthError::Parse(_))));
    }

    #[test]
    fn test_store_token() {
        let email = "test@example.com";
        let token = "test_token";

        let result = store_token(email, token);
        assert!(result.is_ok());
    }
}
