use super::models::*;
use crate::image_analysis::ml_models::models::{ModelConfig, ModelError, ModelManager};

use image::imageops::FilterType;
use ndarray::Array;
use surrealdb::{engine::local::Db, Surreal};

use tract_onnx::prelude::*;

const MODEL_INPUT_SHAPE: (usize, usize) = (28, 28);

impl ImageClassifier {
    pub fn new(model_path: &std::path::Path) -> Result<Self, ModelError> {
        let model = tract_onnx::onnx()
            .model_for_path(model_path)
            .map_err(|e| ModelError::Processing(e.to_string()))?
            .into_optimized()
            .unwrap()
            .into_runnable()
            .unwrap();

        let config = ModelConfig {
            input_shape: MODEL_INPUT_SHAPE,
            class_mapping: vec![
                "abdomen", "angio", "breast", "thorax", "thorax", "hand", "head", "knee",
                "shoulder",
            ],
        };

        Ok(Self { model, config })
    }

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
}



pub async fn process_images_service(
    image_data: String,
    user_name: String,
    model_name: String,
    app_handle: tauri::AppHandle,
    db: &Surreal<Db>,
) -> Result<AnalysisResponse, ModelError> {
    let model_manager = ModelManager::new(app_handle);
    let model_path = model_manager
        .ensure_model_exists(&model_name, &user_name)
        .await
        .map_err(|e| ModelError::FileSystem(e.to_string()))?;

    let processor = ImageClassifier::new(&model_path)?;

    let images: Vec<ImageData> =
        serde_json::from_str(&image_data).map_err(|e| ModelError::Serialization(e.to_string()))?;

    let mut results = Vec::new();
    let mut all_statements = Vec::new();
    let mut added_image_type = Vec::new();

    for img in images {
        let (image_type, confidence) = processor.process_image(&img.data)?;

        results.push(ImageResult {
            filename: img.filename,
            image_type: image_type.clone(),
            confidence,
        });

        match image_type.as_str() {
           "knee" => {
            println!("Knee image detected");
                // let knee_model_path = model_manager
                //     .ensure_model_exists("knee_specific_model", &user_name)
                //     .await
                //     .map_err(|e| ModelError::FileSystem(e.to_string()))?;
                
                // process_knee_image(image_data, &model_path)?;

            },

            "thorax" => {
            println!("Chest image detected");
                // let chest_model_path = model_manager
                //     .ensure_model_exists("chest_specific_model", &user_name)
                //     .await
                //     .map_err(|e| ModelError::FileSystem(e.to_string()))?;
            },
                // process_chest_image(image_data, &model_path)?;

            _ => {}

        }

        if !added_image_type.contains(&image_type) {
            added_image_type.push(image_type.clone());
            let statements = fetch_statements(db, &image_type).await?;
            all_statements.extend(statements);
        } else {
            continue;
        }
    }
    println!("{:?}", results);

    Ok(AnalysisResponse {
        results,
        statements: all_statements,
    })
}

async fn fetch_statements(
    db: &Surreal<Db>,
    body_part: &str,
) -> Result<Vec<StatementResponse>, ModelError> {
    let query = format!(
        "SELECT indication, statement, assessment 
         FROM Statement 
         WHERE body_part = '{}'
         ORDER BY indication;",
        body_part
    );

    db.query(&query)
        .await
        .map_err(|e| ModelError::Database(e.to_string()))?
        .take(0)
        .map_err(|e| ModelError::Database(e.to_string()))
}
