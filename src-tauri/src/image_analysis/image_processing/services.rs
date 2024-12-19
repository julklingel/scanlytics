use super::models::*;
use crate::image_analysis::ml_models::models::{ImageClassifier, ModelError, ModelManager};

use scanlytics_db::{Any, Surreal};
use tauri::Runtime;

pub async fn process_images_service<R: Runtime>(
    image_data: String,
    user_name: String,
    model_name: String,
    app_handle: tauri::AppHandle<R>,
    db: &Surreal<Any>,
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
    db: &Surreal<Any>,
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


#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageFormat;
    use image::{ImageBuffer, Luma};
    use std::{io::Cursor, path::PathBuf};
    use tauri::test::{mock_builder, mock_context};
    use tempfile::TempDir;

   
    struct TestContext {
        db: Surreal<Any>,
        temp_dir: TempDir,
        model_path: PathBuf,
    }

    impl TestContext {
        async fn new() -> Self {

            let db_conn = scanlytics_db::init_db(None, true).await.unwrap();
            let db = db_conn.get().lock().await;
            let db = db.clone();

       
            let temp_dir = TempDir::new().unwrap();
            let model_path = temp_dir.path().join("test_model.onnx");
            std::fs::write(&model_path, b"mock_model_data").unwrap();

         
            db.query(
                "CREATE Statement SET 
                body_part = 'knee', 
                indication = 'test_indication', 
                statement = 'test statement', 
                assessment = 'test assessment'",
            )
            .await
            .unwrap();

            Self {
                db,
                temp_dir,
                model_path,
            }
        }
    }

   
    fn create_test_image() -> Vec<u8> {
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(28, 28);
        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();
        bytes
    }

    fn create_test_image_data(filenames: Vec<(&str, &str)>) -> String {
        let images: Vec<ImageData> = filenames
            .into_iter()
            .map(|(filename, extension)| ImageData {
                filename: filename.to_string(),
                extension: extension.to_string(),
                data: create_test_image(),
            })
            .collect();
        serde_json::to_string(&images).unwrap()
    }

    #[tokio::test]
    async fn test_successful_image_processing() {
        let context = TestContext::new().await;
        
        let app = mock_builder()
            .manage(context.db.clone())
            .manage(context.model_path.clone())
            .build(mock_context(tauri::test::noop_assets()))
            .expect("Failed to build app");

        let app_handle = app.handle().clone();

        let image_data = create_test_image_data(vec![("test1", "jpg"), ("test2", "png")]);

        let result = process_images_service(
            image_data,
            "test_user".to_string(),
            "test_model".to_string(),
            app_handle,
            &context.db,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.results.len(), 2);
        assert!(!response.statements.is_empty());
    }

    #[tokio::test]
    async fn test_invalid_image_data() {
        let context = TestContext::new().await;

        let app = mock_builder()
            .manage(context.db.clone())
            .manage(context.model_path.clone())
            .build(mock_context(tauri::test::noop_assets()))
            .expect("Failed to build app");

        let app_handle = app.handle().clone();

        let result = process_images_service(
            "invalid json".to_string(),
            "test_user".to_string(),
            "test_model".to_string(),
            app_handle,
            &context.db,
        )
        .await;

        assert!(matches!(result, Err(ModelError::Serialization(_))));
    }

    #[tokio::test]
    async fn test_empty_image_list() {
        let context = TestContext::new().await;

        let app = mock_builder()
            .manage(context.db.clone())
            .manage(context.model_path.clone())
            .build(mock_context(tauri::test::noop_assets()))
            .expect("Failed to build app");

        let app_handle = app.handle().clone();

        let empty_list = create_test_image_data(vec![]);

        let result = process_images_service(
            empty_list,
            "test_user".to_string(),
            "test_model".to_string(),
            app_handle,
            &context.db,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.results.len(), 0);
        assert!(response.statements.is_empty());
    }

    #[tokio::test]
    async fn test_duplicate_image_types() {
        let context = TestContext::new().await;

        let app = mock_builder()
            .manage(context.db.clone())
            .manage(context.model_path.clone())
            .build(mock_context(tauri::test::noop_assets()))
            .expect("Failed to build app");

        let app_handle = app.handle().clone();

        
        let image_data = create_test_image_data(vec![
            ("knee1", "jpg"),
            ("knee2", "jpg"),
            ("knee3", "jpg"),
        ]);

        let result = process_images_service(
            image_data,
            "test_user".to_string(),
            "test_model".to_string(),
            app_handle,
            &context.db,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.results.len(), 3);
        assert_eq!(response.statements.len(), 1); 
    }

   


    #[tokio::test]
    async fn test_various_image_extensions() {
        let context = TestContext::new().await;

        let app = mock_builder()
            .manage(context.db.clone())
            .manage(context.model_path.clone())
            .build(mock_context(tauri::test::noop_assets()))
            .expect("Failed to build app");

        let app_handle = app.handle().clone();

        let image_data = create_test_image_data(vec![
            ("image1", "jpg"),
            ("image2", "png"),
            ("image3", "jpeg"),
            ("image4", "bmp"),
        ]);

        let result = process_images_service(
            image_data,
            "test_user".to_string(),
            "test_model".to_string(),
            app_handle,
            &context.db,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.results.len(), 4);
    }

    #[tokio::test]
    async fn test_missing_statements() {
        let context = TestContext::new().await;

        context.db.query("DELETE Statement").await.unwrap();

        let app = mock_builder()
            .manage(context.db.clone())
            .manage(context.model_path.clone())
            .build(mock_context(tauri::test::noop_assets()))
            .expect("Failed to build app");

        let app_handle = app.handle().clone();

        let image_data = create_test_image_data(vec![("test1", "jpg")]);

        let result = process_images_service(
            image_data,
            "test_user".to_string(),
            "test_model".to_string(),
            app_handle,
            &context.db,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.results.len(), 1);
        assert!(response.statements.is_empty());
    }



}
