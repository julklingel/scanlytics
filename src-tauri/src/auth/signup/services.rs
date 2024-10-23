use super::models;
use reqwest::Client as HttpClient;
use zxcvbn::zxcvbn;
use zxcvbn::Score;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use crate::users::services::create_user_service;
use crate::users::models::UserRecord;

pub async fn signup_service(
    db: &Surreal<Db>,
    signup_data: String,
) -> Result<models::SignupResponse, String> {
    let signup_request: models::SignupRequest = serde_json::from_str(&signup_data)
        .map_err(|e| format!("Failed to parse signup request: {}", e))?;

    if signup_request.password != signup_request.confirm_password {
        return Err("Passwords do not match".to_string());
    }

    let password_strength_estimate = zxcvbn(&signup_request.password.to_string(), &[]);

    let score_value = match password_strength_estimate.score() {
        Score::Zero => 0,
        Score::One => 1,
        Score::Two => 2,
        Score::Three => 3,
        Score::Four => 4,
        _ => return Err("zxcvbn entropy score must be in the range 0-4".to_string()),
    };

    if score_value < 3 {
        return Err(
            "This password is weak!".to_string(),
        );
    }

    let signup_record = models::SignupRecord {
        user_password: signup_request.password.clone(),
        user_name: signup_request.full_name.clone(),
        user_email: signup_request.username.clone(),
        user_role: "user".to_string(),
    };

    let user_data = UserRecord {
        name: signup_request.full_name,
        email: signup_request.username,
        password: signup_request.password,
        role: "user".to_string(),
        organization: None,
        patients: None,
        patient_notes: None,
        statements: None,
        images: None,
        reports: None,
    };

    let client = HttpClient::new();
    let response = client
        .post("https://scanlyticsbe.fly.dev/auth/user_signup")
        .json(&signup_record)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    println!("Response status: {}", response.status());

    if response.status().is_success() {
        let signup_response: models::SignupResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        create_user_service(user_data, db).await?;
        Ok(signup_response)
    } else {
        Err(format!("Signup failed: {}", response.status()))
    }
}
