//! # Medical Reports Module
//! 
//! This module handles all functionality related to medical reports including:
//! - Report creation and management
//! - Image processing and storage
//! - Report retrieval and querying
//! 
//! ## Components
//! 
//! - [`controller`]: Tauri command handlers for report operations
//! - [`services`]: Report management business logic
//! - [`models`]: Report-related data structures
//! 
//! ## Main Features
//! 
//! - Create medical reports with multiple images
//! - Retrieve reports with patient and user information
//! - Manage report-image relationships
//! - Secure file storage and retrieval



pub mod controller;
pub mod models;
pub mod services;
