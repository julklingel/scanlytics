use super::models::{SignupError, SignupRequest, SignupResponse, SignupServerRequest};
use reqwest::Client as HttpClient;
use zxcvbn::{zxcvbn, Score};

use scanlytics_db::{Surreal, Db};



use crate::users::services::create_user_service;
use crate::users::models::UserRecord;

pub async fn signup_service(
    db: &Surreal<Db>,
    signup_data: String,
) -> Result<SignupResponse, SignupError> {
 
    let signup_request: SignupRequest = serde_json::from_str(&signup_data)
        .map_err(|e| SignupError::ParseError(e.to_string()))?;

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

    let response = send_signup_to_server(&signup_record_server).await?;

    create_user_service(user_data, db)
        .await
        .map_err(|e| SignupError::DatabaseError(e.to_string()))?;

    Ok(response)
}

async fn send_signup_to_server(signup_record: &SignupServerRequest) -> Result<SignupResponse, SignupError> {
    let client = HttpClient::new();
    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/user_signup")
        .json(signup_record)
        .send()
        .await
        .map_err(|e| SignupError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        response
            .json()
            .await
            .map_err(|e| SignupError::ParseError(e.to_string()))
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
        _ => return Err(SignupError::ValidationError(
            "Invalid password strength score".into()
        )),
    };

    if score < 3 {
        return Err(SignupError::WeakPassword(
            "Password does not meet minimum strength requirements".into()
        ));
    }

    Ok(())
}
