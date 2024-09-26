// src/auth_middleware.rs

use keyring::Entry;
use reqwest::Client;
use serde_json::json;

pub async fn validate_token(username: &str) -> Result<(), String> {
    let keyring_entry = Entry::new("Scanlytics", username)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
    
    let token = keyring_entry.get_password()
        .map_err(|e| format!("Failed to retrieve token: {}", e))?;

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

#[macro_export]
macro_rules! with_auth {
    ($func:ident) => {
        #[tauri::command]
        pub async fn $func(
            username: String,
            db: tauri::State<'_, tokio::sync::RwLock<surrealdb::Surreal<surrealdb::engine::remote::ws::Client>>>,
            #[allow(unused_variables)] args: serde_json::Value,
        ) -> Result<serde_json::Value, String> {
            crate::auth_middleware::validate_token(&username).await?;
            
            let db = db.read().await;
            let result = crate::commands::$func(&db, args).await?;
            Ok(serde_json::to_value(result).map_err(|e| e.to_string())?)
        }
    };
}
