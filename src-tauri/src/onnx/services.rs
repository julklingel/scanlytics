use super::models;
use image::imageops::FilterType;
use ndarray::Array;
use std::fs;
use tract_onnx::prelude::*;

use tauri::Manager;

pub async fn test_onnx_model_service(
    image_path: String,
    model_type: String,
    app_handle: tauri::AppHandle,
) -> Result<models::ONNXResponse, String> {
    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data directory: {}", e))?;

    let load_dir = app_local_data_dir.join("onnx");

    if !load_dir.exists() {
        fs::create_dir_all(&load_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let model_path = load_dir.join("MNST_med.onnx");

    if !model_path.exists() {
        return Err(format!(
            "ONNX model file not found at {:?}",
            model_path
                .canonicalize()
                .unwrap_or(model_path.to_path_buf())
        ));
    }

    let model = tract_onnx::onnx()
        .model_for_path(model_path)
        .unwrap()
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap();

    let image_path = image_path.trim_matches('"');
    println!("Image path {:?}", image_path);

    if image_path.is_empty() {
        return Err(format!("Image file not found at {:?}", image_path));
    }
    let img = image::open(&image_path)
        .map_err(|e| e.to_string())?
        .resize_exact(28, 28, FilterType::Lanczos3)
        .to_luma8();

    let img_array: Array<f32, _> = Array::from_shape_fn((1, 1, 28, 28), |(_, _, y, x)| {
        (img[(x as _, y as _)][0] as f32 - 127.5) / 127.5
    });

    let (vec, offset) = img_array.into_raw_vec_and_offset();
    let input =
        tract_ndarray::Array4::from_shape_vec((1, 1, 28, 28), vec).map_err(|e| e.to_string())?;
    let input_tensor = input.into_tensor();

    let result = model
        .run(tvec!(input_tensor.into()))
        .map_err(|e| e.to_string())?;

    let output = result[0]
        .to_array_view::<f32>()
        .map_err(|e| e.to_string())?;
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

    Ok(models::ONNXResponse {
        image_type: image_type.to_string(),
        confidence: *confidence,
    })
}



pub async fn process_image_service(
    image_data: models::ImageData,

) -> Result<models::ONNXResponse, String> {
    println!("Processing image data {:?}", image_data);

    let mock_response = models::ONNXResponse {
        image_type: "AbdomenCT".to_string(),
        confidence: 0.99,
    };

    return Ok(mock_response);

}