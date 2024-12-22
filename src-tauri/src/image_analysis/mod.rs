//! # Image Analysis Module
//! 
//! Handles medical image processing and analysis, including:
//! - Image classification using ML models
//! - Model management and downloading
//! - Statement generation based on image types
//! 
//! ## Features
//! 
//! - **ML Model Integration**: ONNX model support
//! - **Image Processing**: Preprocessing and classification
//! - **Model Management**: Automatic downloading and caching
//! - **Statement Generation**: Medical statement retrieval
//! 
//! ## Components
//! 
//! - [`image_processing`]: Image processing and analysis
//! - [`ml_models`]: Machine learning model management

pub mod image_processing;
pub mod ml_models;