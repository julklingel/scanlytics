use super::models;
use image::imageops::FilterType;
use ndarray::{Array, ArrayView};
use tauri::AppHandle;
use tract_onnx::prelude::*;
use log::{info, error};
use std::fs;

use tauri::Manager;


pub async fn test_onnx_model_service(image_path: String, app_handle: tauri::AppHandle) -> Result<models::ONNXResponse, String> {

    
    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data directory: {}", e))?;

    println!("app_local_data_dir: {:?}", app_local_data_dir);


    let load_dir = app_local_data_dir.join("onnx");

    println!("load_dir: {:?}", load_dir);

    if !load_dir.exists() {
        info!("Creating ONNX directory");
        fs::create_dir_all(&load_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let model_path = load_dir.join("MNST_med.onnx");


    println!("model_path: {:?}", model_path);

    if !model_path.exists() {
        error!("ONNX model file not found at {:?}", model_path);
        return Err(format!(
            "ONNX model file not found at {:?}",
            model_path
                .canonicalize()
                .unwrap_or(model_path.to_path_buf())
        ));
    }

    // Load and prepare the model
    let model = tract_onnx::onnx()
        .model_for_path(model_path)
        .unwrap()
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap();



    println!("model: {:?}", model);

    let image_path = app_local_data_dir.join("saved_images").join("Image:p49wval1te4tf7lboyj6.bmp");

    println!("image_path: {:?}", image_path);

    // Load and preprocess the image
    let img = image::open(&image_path)
        .map_err(|e| e.to_string())?
        .resize_exact(28, 28, FilterType::Lanczos3)
        .to_luma8();

    let img_array: Array<f32, _> = Array::from_shape_fn((1, 1, 28, 28), |(_, _, y, x)| {
        (img[(x as _, y as _)][0] as f32 - 127.5) / 127.5
    });

    // Convert the array to a Tensor
    let (vec, offset) = img_array.into_raw_vec_and_offset();
    let input =
        tract_ndarray::Array4::from_shape_vec((1, 1, 28, 28), vec).map_err(|e| e.to_string())?;
    let input_tensor = input.into_tensor();

    // Run inference
    let result = model
        .run(tvec!(input_tensor.into()))
        .map_err(|e| e.to_string())?;

    // Process the output
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
