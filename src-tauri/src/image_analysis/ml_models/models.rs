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

const SERVICE_NAME: &str = "com.scanlytics.dev";
const API_BASE_URL: &str = "https://scanlyticsbe.fly.dev";

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub filename: String,
    pub extension: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub filename: String,
    pub image_type: String,
    pub confidence: f32,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct ModelConfig {
    pub input_shape: (usize, usize),
    pub class_mapping: Vec<&'static str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct ImageClassifier {
    #[cfg(not(feature = "test-utils"))]
    pub(crate) model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
     #[cfg(feature = "test-utils")]
    pub(crate) model: (),
    pub(crate) config: ModelConfig,
}

#[derive(Debug)]
pub struct ModelManager<R: Runtime> {
    pub client: Client,
    app_handle: tauri::AppHandle<R>,
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("File system error: {0}")]
    FileSystem(String),
    #[error("Model processing error: {0}")]
    Processing(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Image processing error: {0}")]
    Image(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

async fn get_stored_token(user_name: &str) -> Result<String, ModelError> {
    let entry = Entry::new(SERVICE_NAME, user_name.trim())
        .map_err(|e| ModelError::Auth(format!("Failed to access keyring: {}", e)))?;

    entry
        .get_password()
        .map_err(|e| ModelError::Auth(format!("Failed to retrieve token: {}", e)))
}

impl<R: Runtime> ModelManager<R> {
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



impl ImageClassifier {
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

        let model = model.into_optimized().unwrap().into_runnable().unwrap();

        let config = ModelConfig {
            input_shape: input_shape_size,
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
                class_mapping: vec![
                    "abdomen", "angio", "breast", "thorax", "thorax", "hand", "head", "knee",
                    "shoulder",
                ],
            },
        })
    }

    #[cfg(not(feature = "test-utils"))]
    pub fn process_image(&self, image_data: &[u8]) -> Result<(String, f32), ModelError> {
        let (width, height) = self.config.input_shape;

        let img = image::load_from_memory(image_data)
            .map_err(|e| ModelError::Image(e.to_string()))?
            .resize_exact(width as u32, height as u32, FilterType::Lanczos3)
            .to_luma8();

        let img_array: Array<f32, _> =
            Array::from_shape_fn((1, 1, width, height), |(_, _, y, x)| {
                (img[(x as _, y as _)][0] as f32 - 127.5) / 127.5
            });

        let (vec, _) = img_array.into_raw_vec_and_offset();
        let input = tract_ndarray::Array4::from_shape_vec((1, 1, 28, 28), vec)
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let result = self
            .model
            .run(tvec!(input.into_tensor().into()))
            .map_err(|e| ModelError::Processing(e.to_string()))?;

        let output = result[0]
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