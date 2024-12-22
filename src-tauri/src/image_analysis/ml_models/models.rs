//! # ML Model Implementation
//! 
//! Core implementation of machine learning model management and image classification.
//! This module provides the fundamental structures and implementations for:
//! - Model configuration and initialization
//! - Image processing and classification
//! - Model file management and downloading
//! - Secure token handling

use image::imageops::FilterType;
use keyring::Entry;
use ndarray::Array;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tauri::Manager;
use thiserror::Error;
use tract_onnx::prelude::*;
use tauri::Runtime;

/// Service name for keyring operations
const SERVICE_NAME: &str = "com.scanlytics.dev";
/// Base URL for ML model API
const API_BASE_URL: &str = "https://scanlyticsbe.fly.dev";

/// Input image data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    /// Original filename
    pub filename: String,
    /// File extension (e.g., "jpg", "png")
    pub extension: String,
    /// Raw image data
    pub data: Vec<u8>,
}

/// Classification result structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResult {
    /// Original filename
    pub filename: String,
    /// Classified image type
    pub image_type: String,
    /// Classification confidence score (0.0 to 1.0)
    pub confidence: f32,
}

/// ML model configuration
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct ModelConfig {
    /// Input image dimensions (height, width)
    pub input_shape: (usize, usize),
    /// Number of input channels (1 for grayscale, 3 for RGB)
    pub channels: usize,
    /// Available classification categories
    pub class_mapping: Vec<&'static str>,
}

/// Image classification model wrapper
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct ImageClassifier {
    /// ONNX model for production use
    #[cfg(not(feature = "test-utils"))]
    pub(crate) model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
    /// Mock model for testing
    #[cfg(feature = "test-utils")]
    pub(crate) model: (),
    /// Model configuration
    pub(crate) config: ModelConfig,
}

/// Model management and downloading
#[derive(Debug)]
pub struct ModelManager<R: Runtime> {
    /// HTTP client for model downloading
    pub client: Client,
    /// Tauri application handle
    app_handle: tauri::AppHandle<R>,
}

/// Possible errors during model operations
#[derive(Debug, Error)]
pub enum ModelError {
    /// Authentication and token-related errors
    #[error("Authentication error: {0}")]
    Auth(String),
    /// Network communication errors
    #[error("Network error: {0}")]
    Network(String),
    /// File system operation errors
    #[error("File system error: {0}")]
    FileSystem(String),
    /// Model processing and inference errors
    #[error("Model processing error: {0}")]
    Processing(String),
    /// Database operation errors
    #[error("Database error: {0}")]
    Database(String),
    /// Image processing errors
    #[error("Image processing error: {0}")]
    Image(String),
    /// Data serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Retrieves stored authentication token
///
/// # Arguments
///
/// * `user_name` - Username for token retrieval
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(String)` - Retrieved token
/// * `Err(ModelError)` - Authentication error details
async fn get_stored_token(user_name: &str) -> Result<String, ModelError> {
    let entry = Entry::new(SERVICE_NAME, user_name.trim())
        .map_err(|e| ModelError::Auth(format!("Failed to access keyring: {}", e)))?;

    entry
        .get_password()
        .map_err(|e| ModelError::Auth(format!("Failed to retrieve token: {}", e)))
}

impl<R: Runtime> ModelManager<R> {
    /// Creates a new model manager instance
    ///
    /// # Arguments
    ///
    /// * `app_handle` - Tauri application handle
    pub fn new(app_handle: tauri::AppHandle<R>) -> Self {
        Self {
            client: Client::new(),
            app_handle,
        }
    }

    pub async fn ensure_model_exists(&self, model_name: &str, user_name: &str) -> Result<PathBuf, ModelError> {
         #[cfg(feature = "test-utils")] {
            if let Some(state) = self.app_handle.try_state::<PathBuf>() {
                let path = state.inner().clone();
                Ok(path.clone())
            } else {
                Ok(PathBuf::from("test_model.onnx"))
            }
        }

        #[cfg(not(feature = "test-utils"))] {
            let model_path = self.get_model_path(model_name)?;
            
            if !model_path.exists() {
                self.download_model(model_name, user_name).await?;
            }
            
            Ok(model_path)
        }
    }

    /// Ensures model file exists, downloading if necessary
    ///
    /// # Arguments
    ///
    /// * `model_name` - Name of the model to ensure
    /// * `user_name` - Username for authentication
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `Ok(PathBuf)` - Path to the model file
    /// * `Err(ModelError)` - Error details if operation fails

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
    /// Downloads a model file
    ///
    /// # Arguments
    ///
    /// * `model_name` - Name of the model to download
    /// * `user_name` - Username for authentication
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure
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



impl ImageClassifier {
    /// Creates a new image classifier instance
    ///
    /// # Arguments
    ///
    /// * `model_path` - Path to the ONNX model file
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `Ok(Self)` - Initialized classifier
    /// * `Err(ModelError)` - Initialization error details
    #[cfg(not(feature = "test-utils"))]
    pub fn new(model_path: &std::path::Path) -> Result<Self, ModelError> {
        let model = tract_onnx::onnx()
            .model_for_path(model_path)
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let input_fact = model
            .input_fact(0)
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let dimensions: Vec<_> = input_fact.shape.dims().collect();

        let img_height = dimensions[2].to_string().parse::<usize>().unwrap_or(28);
        let img_width = dimensions[3].to_string().parse::<usize>().unwrap_or(28);
        let input_shape_size = (img_height, img_width);
        let channels = dimensions[1].to_string().parse::<usize>().unwrap_or(1);
      

        let model = model.into_optimized().unwrap().into_runnable().unwrap();

        let config = ModelConfig {
            input_shape: input_shape_size,
            channels: channels,
            class_mapping: vec![
                "abdomen", "angio", "breast", "thorax", "thorax", "hand", "head", "knee",
                "shoulder",
            ],
        };

        Ok(Self { model, config })
    }

    
    #[cfg(feature = "test-utils")]
    pub fn new(_model_path: &std::path::Path) -> Result<Self, ModelError> {
        Ok(Self {
            model: (),
            config: ModelConfig {
                input_shape: (28, 28),
                channels: 1,
                class_mapping: vec![
                    "abdomen", "angio", "breast", "thorax", "thorax", "hand", "head", "knee",
                    "shoulder",
                ],
            },
        })
    }

    /// Processes an image for classification
    ///
    /// # Arguments
    ///
    /// * `image_data` - Raw image data
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `Ok((String, f32))` - (Image type, confidence score)
    /// * `Err(ModelError)` - Processing error details
    #[cfg(not(feature = "test-utils"))]
    pub fn process_image(&self, image_data: &[u8]) -> Result<(String, f32), ModelError> {
        let (width, height) = self.config.input_shape;
        let channels = self.config.channels;

        let img = image::load_from_memory(image_data)
            .map_err(|e| ModelError::Image(e.to_string()))?
            .resize_exact(width as u32, height as u32, FilterType::Lanczos3)
            .to_luma8();

        let img_array: Array<f32, _> =
            Array::from_shape_fn((1, channels, width, height), |(_, _, y, x)| {
                (img[(x as _, y as _)][0] as f32 - 127.5) / 127.5
            });

        let (vec, _) = img_array.into_raw_vec_and_offset();

        let input = tract_ndarray::Array4::from_shape_vec((1, channels, width, height), vec)
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let result = self.model.run(tvec!(input.into_tensor().into()))
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let output: ndarray::ArrayBase<ndarray::ViewRepr<&f32>, ndarray::Dim<ndarray::IxDynImpl>> = result[0]
            .to_array_view::<f32>()
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let (class_idx, confidence) = output
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        let image_type = self
            .config
            .class_mapping
            .get(class_idx)
            .copied()
            .unwrap_or("unknown");

        Ok((image_type.to_string(), *confidence))
    }

     #[cfg(feature = "test-utils")]
    pub fn process_image(&self, _image_data: &[u8]) -> Result<(String, f32), ModelError> {
        Ok(("knee".to_string(), 0.95))
    }
}