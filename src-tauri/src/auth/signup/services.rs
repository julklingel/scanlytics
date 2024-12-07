use super::models::{SignupError, SignupRequest, SignupResponse, ServerResponse, SignupServerRequest};
use reqwest::Client as HttpClient;
use zxcvbn::{zxcvbn, Score};

use scanlytics_db::{Any, Surreal};

use crate::users::models::UserRecord;
use crate::users::services::create_user_service;

pub async fn signup_service(
    db: &Surreal<Any>,
    signup_data: String,
    base_url: Option<String>,
) -> Result<SignupResponse, SignupError> {
    let signup_request: SignupRequest =
        serde_json::from_str(&signup_data).map_err(|e| SignupError::ParseError(e.to_string()))?;

    signup_request.validate()?;
    validate_password_strength(&signup_request.password)?;

    let signup_record_server: SignupServerRequest = signup_request.clone().into();

    let user_data = UserRecord {
        name: signup_request.full_name,
        email: signup_request.user_email,
        role: "user".to_string(),
        organization: None,
        patients: None,
        patient_notes: None,
        statements: None,
        images: None,
        reports: None,
    };

    let response = send_signup_to_server(&signup_record_server, base_url).await?;

    create_user_service(user_data, db)
        .await
        .map_err(|e| SignupError::DatabaseError(e.to_string()))?;

    Ok(response)
}

async fn send_signup_to_server(
    signup_record: &SignupServerRequest,
    base_url: Option<String>,
) -> Result<SignupResponse, SignupError> {
    let client = HttpClient::new();
    let url = base_url.unwrap_or_else(|| "https://scanlyticsbe.fly.dev".to_string())
        + "/auth/user_signup";

    let response = client
        .post(&url)
        .json(signup_record)
        .send()
        .await
        .map_err(|e| SignupError::NetworkError(e.to_string()))?;



    if response.status().is_success() {
        
        let response_array: Vec<ServerResponse> = response
            .json()
            .await
            .map_err(|e| SignupError::ParseError(e.to_string()))?;
        
        
        let message = response_array
            .iter()
            .find_map(|r| r.message.clone())
            .unwrap_or_else(|| "User registered successfully".to_string());

        Ok(SignupResponse { message })
    } else {
        Err(SignupError::ServerError(format!(
            "Signup failed: {}",
            response.status()
        )))
    }
}

fn validate_password_strength(password: &str) -> Result<(), SignupError> {
    let estimate = zxcvbn(password, &[]);

    let score = match estimate.score() {
        Score::Zero => 0,
        Score::One => 1,
        Score::Two => 2,
        Score::Three => 3,
        Score::Four => 4,
        _ => {
            return Err(SignupError::ValidationError(
                "Invalid password strength score".into(),
            ))
        }
    };

    if score < 3 {
        return Err(SignupError::WeakPassword(
            "Password does not meet minimum strength requirements".into(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_test_db() -> Surreal<Any> {
        let db_conn = scanlytics_db::init_db(None, true).await.unwrap();
        let db = db_conn.get().lock().await;
        db.clone()
    }

    #[tokio::test]
    async fn test_successful_signup() {
        let mock_server = MockServer::start().await;
        let mock_url = mock_server.uri();
    
        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "test@example.com",
            "password": "StrongP@ssword123!",
            "confirm_password": "StrongP@ssword123!"
        });
    
        let expected_server_request = json!({
            "user_name": "Test User",
            "user_email": "test@example.com",
            "user_password": "StrongP@ssword123!",
            "user_role": "user"
        });
    
        let mock_response = json!([
            {
                "message": "User registered successfully"
            },
            {
                "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "token_type": "bearer"
            }
        ]);
    
        Mock::given(method("POST"))
            .and(path("/auth/user_signup"))
            .and(header("content-type", "application/json"))
            .and(body_json(&expected_server_request))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(&mock_response)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&mock_server)
            .await;
    
        let db = setup_test_db().await;
        let response = signup_service(&db, signup_data.to_string(), Some(mock_url)).await;
    
        match &response {
            Ok(r) => println!("Signup succeeded: {:?}", r),
            Err(e) => println!("Signup failed with error: {:?}", e),
        }
    
        assert!(response.is_ok());
        if let Ok(signup_response) = response {
            assert_eq!(signup_response.message, "User registered successfully");
        }
    }
    

    #[tokio::test]
    async fn test_invalid_json_data() {
        let db = setup_test_db().await;
        let invalid_json = "{ invalid_json: }".to_string();

        let result = signup_service(&db, invalid_json, None).await;

        assert!(matches!(result, Err(SignupError::ParseError(_))));
    }

    #[tokio::test]
    async fn test_password_mismatch() {
        let db = setup_test_db().await;
        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "test@example.com",
            "password": "StrongP@ssword123!",
            "confirm_password": "DifferentPassword123!"
        });

        let result = signup_service(&db, signup_data.to_string(), None).await;

        assert!(matches!(result, Err(SignupError::PasswordMismatch)));
    }

    #[tokio::test]
    async fn test_weak_password() {
        let db = setup_test_db().await;
        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "test@example.com",
            "password": "weak",
            "confirm_password": "weak"
        });

        let result = signup_service(&db, signup_data.to_string(), None).await;

        assert!(matches!(result, Err(SignupError::WeakPassword(_))));
    }

    #[tokio::test]
    async fn test_empty_fields() {
        let db = setup_test_db().await;
        let signup_data = json!({
            "full_name": "",
            "user_email": "test@example.com",
            "password": "StrongP@ssword123!",
            "confirm_password": "StrongP@ssword123!"
        });

        let result = signup_service(&db, signup_data.to_string(), None).await;

        assert!(matches!(result, Err(SignupError::ValidationError(_))));

        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "",
            "password": "StrongP@ssword123!",
            "confirm_password": "StrongP@ssword123!"
        });

        let result = signup_service(&db, signup_data.to_string(), None).await;

        assert!(matches!(result, Err(SignupError::ValidationError(_))));
    }

    #[tokio::test]
    async fn test_server_error_response() {
        let mock_server = MockServer::start().await;
        let mock_url = mock_server.uri();

        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "test@example.com",
            "password": "StrongP@ssword123!",
            "confirm_password": "StrongP@ssword123!"
        });

        Mock::given(method("POST"))
            .and(path("/auth/user_signup"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let db = setup_test_db().await;
        let result = signup_service(&db, signup_data.to_string(), Some(mock_url)).await;

        assert!(matches!(result, Err(SignupError::ServerError(_))));
    }

    #[tokio::test]
    async fn test_network_error() {
        let db = setup_test_db().await;
        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "test@example.com",
            "password": "StrongP@ssword123!",
            "confirm_password": "StrongP@ssword123!"
        });

        let result = signup_service(
            &db,
            signup_data.to_string(),
            Some("http://invalid-url".to_string()),
        )
        .await;

        assert!(matches!(result, Err(SignupError::NetworkError(_))));
    }

    #[tokio::test]
    async fn test_malformed_server_response() {
        let mock_server = MockServer::start().await;
        let mock_url = mock_server.uri();

        let signup_data = json!({
            "full_name": "Test User",
            "user_email": "test@example.com",
            "password": "StrongP@ssword123!",
            "confirm_password": "StrongP@ssword123!"
        });

        Mock::given(method("POST"))
            .and(path("/auth/user_signup"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let db = setup_test_db().await;
        let result = signup_service(&db, signup_data.to_string(), Some(mock_url)).await;

        assert!(matches!(result, Err(SignupError::ParseError(_))));
    }
}
