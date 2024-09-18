use super::models;
use reqwest::Client as HttpClient;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use zxcvbn::zxcvbn;
use zxcvbn::Score;

pub async fn signup_service(
    _db: &Surreal<Client>,
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
            "This password seems weak - make it stronger so your data stays secure.".to_string(),
        );
    }

    let client = HttpClient::new();
    let response = client
        .post("https://fp.com/signup")
        .json(&signup_request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    println!("Response status: {}", response.status());

    if response.status().is_success() {
        let signup_response: models::SignupResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        println!("Signup successful: {:?}", signup_response);
        Ok(signup_response)
    } else {
        Err(format!("Signup failed: {}", response.status()))
    }
}
