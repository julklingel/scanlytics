use super::models::*;
use crate::image_analysis::ml_models::models::{
    ImageClassifier, ModelError, ModelManager,
};

use surrealdb::{engine::local::Db, Surreal};

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

        // Subsequent processing based on image type
        match image_type.as_str() {
            "knee" => {
                // let knee_model_path = model_manager
                //     .ensure_model_exists("knee_specific_model", &user_name)
                //     .await
                //     .map_err(|e| ModelError::FileSystem(e.to_string()))?;

                // process_knee_image(image_data, &model_path)?;
            }

            "thorax" => {
                // let chest_model_path = model_manager
                //     .ensure_model_exists("chest_specific_model", &user_name)
                //     .await
                //     .map_err(|e| ModelError::FileSystem(e.to_string()))?;
            }
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
