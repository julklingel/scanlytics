use super::models::*;
use keyring::Entry;
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tauri::Manager;


const SERVICE_NAME: &str = "com.scanlytics.dev";
const API_BASE_URL: &str = "https://scanlyticsbe.fly.dev";




pub struct ModelManager {
    client: Client,
    app_handle: tauri::AppHandle,
}

impl ModelManager {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            client: Client::new(),
            app_handle,
        }
    }

    pub async fn ensure_model_exists(&self, model_name: &str, user_name: &str) -> Result<PathBuf, ModelError> {
        let model_path = self.get_model_path(model_name)?;
        
        if !model_path.exists() {
            self.download_model(model_name, user_name).await?;
        }
        
        Ok(model_path)
    }

    fn get_model_path(&self, model_name: &str) -> Result<PathBuf, ModelError> {
        let app_local_data_dir = self.app_handle
            .path()
            .app_local_data_dir()
            .map_err(|e| ModelError::FileSystem(e.to_string()))?;

        let onnx_dir = app_local_data_dir.join("onnx");
        fs::create_dir_all(&onnx_dir)
            .map_err(|e| ModelError::FileSystem(e.to_string()))?;

        Ok(onnx_dir.join(format!("{}.onnx", model_name)))
    }

    async fn download_model(&self, model_name: &str, user_name: &str) -> Result<(), ModelError> {
        let token = get_stored_token(user_name).await?;
        let url = self.get_presigned_url(model_name, &token).await?;
        let bytes = self.download_model_file(&url).await?;
        self.save_model_file(model_name, &bytes)?;
        Ok(())
    }

    async fn get_presigned_url(&self, model_name: &str, token: &str) -> Result<String, ModelError> {
        let response = self.client
            .post(&format!("{}/ml_models/", API_BASE_URL))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "model_name": model_name }))
            .send()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ModelError::Auth("Invalid token".to_string()));
        }

        let url_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        url_response["url"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| ModelError::Network("Invalid URL response".to_string()))
    }

    async fn download_model_file(&self, url: &str) -> Result<Vec<u8>, ModelError> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| ModelError::Network(e.to_string()))
    }

    fn save_model_file(&self, model_name: &str, bytes: &[u8]) -> Result<(), ModelError> {
        let model_path = self.get_model_path(model_name)?;
        let mut file = File::create(&model_path)
            .map_err(|e| ModelError::FileSystem(e.to_string()))?;

        file.write_all(bytes)
            .map_err(|e| ModelError::FileSystem(e.to_string()))
    }
}

async fn get_stored_token(user_name: &str) -> Result<String, ModelError> {
    let entry = Entry::new(SERVICE_NAME, user_name.trim())
        .map_err(|e| ModelError::Auth(format!("Failed to access keyring: {}", e)))?;

    entry
        .get_password()
        .map_err(|e| ModelError::Auth(format!("Failed to retrieve token: {}", e)))
}


