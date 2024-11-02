use super::models;
use image::imageops::FilterType;
use keyring::Entry;
use ndarray::Array;
use reqwest::StatusCode;
use serde_json::json;
use surrealdb::engine::local::Db;
use std::fs::File;
use std::fs::{self};
use std::path::PathBuf;
use tauri::Manager;
use surrealdb::Surreal;

use tract_onnx::prelude::*;


async fn ensure_directory_exists(path: PathBuf) -> Result<PathBuf, models::DownloadError> {
    fs::create_dir_all(&path)
        .map_err(|e| models::DownloadError::FileSystemError(e.to_string()))?;
    Ok(path)
}

async fn get_stored_token(username: &str) -> Result<String, models::DownloadError> {
    let entry = Entry::new("com.scanlytics.dev", username)
        .map_err(|e| models::DownloadError::AuthError(e.to_string()))?;
    entry
        .get_password()
        .map_err(|e| models::DownloadError::AuthError(e.to_string()))
}

pub async fn download_model(
    model_name: &str,
    app_handle: &tauri::AppHandle,
    user_name: String,
) -> Result<(), models::DownloadError> {
    let stored_token = get_stored_token(&user_name).await?;
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://scanlyticsbe.fly.dev/ml_models/")
        .header("Authorization", format!("Bearer {}", stored_token))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json!({"model_name": model_name})).unwrap())
        .send()
        .await
        .map_err(|e| models::DownloadError::NetworkError(e.to_string()))?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(models::DownloadError::AuthError("Invalid token".to_string()));
    }

    let url_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| models::DownloadError::NetworkError(e.to_string()))?;

    let presigned_url = url_response["url"]
        .as_str()
        .ok_or_else(|| models::DownloadError::NetworkError("Invalid URL response".to_string()))?;

    let model_response = client
        .get(presigned_url)
        .send()
        .await
        .map_err(|e| models::DownloadError::NetworkError(e.to_string()))?;

    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| models::DownloadError::FileSystemError(e.to_string()))?;

    let onnx_dir = ensure_directory_exists(app_local_data_dir.join("onnx")).await?;
    let file_path = onnx_dir.join(format!("{}.onnx", model_name));

    let mut file = File::create(&file_path)
        .map_err(|e| models::DownloadError::FileSystemError(e.to_string()))?;

    let bytes = model_response
        .bytes()
        .await
        .map_err(|e| models::DownloadError::NetworkError(e.to_string()))?;

    std::io::copy(&mut std::io::Cursor::new(bytes), &mut file)
        .map_err(|e| models::DownloadError::FileSystemError(e.to_string()))?;

    Ok(())
}

pub async fn process_images_service(
    image_data: String,
    user_name: String,
    model_name: String,
    app_handle: tauri::AppHandle,
    db: &Surreal<Db>,
) -> Result<models::ONNXResponse, String> {
    let app_local_data_dir = app_handle
    .path()
    .app_local_data_dir()
    .map_err(|e| format!("Failed to get app local data directory: {}", e))?;
    
    let load_dir = app_local_data_dir.join("onnx");
    let model_path = load_dir.join(format!("{}.onnx", model_name));
    
    if !model_path.exists() {
        println!("Model not found. Downloading... {}", model_name);
        download_model(&model_name, &app_handle, user_name)
            .await
            .map_err(|e| format!("Failed to download model: {}", e))?;
    }
    
    let image_data: Vec<models::ImageData> = serde_json::from_str(&image_data)
        .map_err(|e| format!("Failed to parse image data: {}", e))?;
    
    let model = tract_onnx::onnx()
        .model_for_path(model_path)
        .map_err(|e| format!("Failed to load ONNX model: {}", e))?
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap();
    
    let mut results = Vec::new();
    
    for img_data in image_data {
        let img = image::load_from_memory(&img_data.data)
            .map_err(|e| format!("Failed to load image from memory: {}", e))?
            .resize_exact(28, 28, FilterType::Lanczos3)
            .to_luma8();
    
        let img_array: Array<f32, _> = Array::from_shape_fn((1, 1, 28, 28), |(_, _, y, x)| {
            (img[(x as _, y as _)][0] as f32 - 127.5) / 127.5
        });
    
        let (vec, offset) = img_array.into_raw_vec_and_offset();
        let input = tract_ndarray::Array4::from_shape_vec((1, 1, 28, 28), vec)
            .map_err(|e| e.to_string())?;
    
        let input_tensor = input.into_tensor();
    
        let result = model
            .run(tvec!(input_tensor.into()))
            .map_err(|e| format!("Failed to run model: {}", e))?;
    
        let output = result[0]
            .to_array_view::<f32>()
            .map_err(|e| format!("Failed to get output: {}", e))?;
        let (class_idx, confidence) = output
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
    
        let image_type = match class_idx {
            0 => "abdomen",
            1 => "angio",
            2 => "breast",
            3 => "thorax",
            4 => "thorax",
            5 => "hand",
            6 => "head",
            7 => "knee",
            8 => "shoulder",
            _ => "unknown",
        };


    
        results.push(models::ImageResult {
            filename: img_data.filename,
            image_type: image_type.to_string(),
            confidence: *confidence,
        });
    }

    let mut all_statements = Vec::new();
    
    
    for result in &results {
        let query = format!("
            SELECT 
                indication,
                statement,
                assessment
            FROM Statement 
            WHERE body_part = '{}'
            ORDER BY indication;
        ", result.image_type);

        let statements: Vec<models::StatementResponse> = db.query(&query)
            .await
            .map_err(|e| format!("Database query failed: {}", e))?
            .take(0)
            .map_err(|e| format!("Failed to extract results: {}", e))?;

        all_statements.extend(statements);
    }

    println!("Results: {:?}", results);
    
    Ok(models::ONNXResponse { 
        results,
        statements: all_statements,
    })
}
