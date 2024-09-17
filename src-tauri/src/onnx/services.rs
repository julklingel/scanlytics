use super::models;
use image::imageops::FilterType;
use ndarray::Array;
use tract_onnx::prelude::*;
use tauri::Manager;


pub async fn process_images_service(
    image_data: String,
    app_handle: tauri::AppHandle,
) -> Result<models::ONNXResponse, String> {
    
    let image_data: Vec<models::ImageData> = serde_json::from_str(&image_data)
        .map_err(|e| format!("Failed to parse image data: {}", e))?;

    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data directory: {}", e))?;

    let load_dir = app_local_data_dir.join("onnx");
    let model_path = load_dir.join("MNST_med.onnx");


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
        let input = tract_ndarray::Array4::from_shape_vec((1, 1, 28, 28), vec).map_err(|e| e.to_string())?;

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
            0 => "AbdomenCT",
            1 => "AngioXR",
            2 => "BreastMRI",
            3 => "ChestCT",
            4 => "ChestXR",
            5 => "HandXR",
            6 => "HeadCT",
            7 => "KneeXR",
            8 => "ShoulderXR",
            _ => "Unknown",
        };

        results.push(models::ImageResult {
            filename: img_data.filename,
            image_type: image_type.to_string(),
            confidence: *confidence,
        });
    }
    

    println!("Results: {:?}", results);

    Ok(models::ONNXResponse { results })
}
