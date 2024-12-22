//! # Patient Notes Module
//! 
//! This module handles all functionality related to patient medical notes including:
//! - Creation and management of patient notes
//! - Note retrieval and querying
//! - Note updates and modifications
//! - Note deletion
//! - Relationships between notes, patients, and medical staff
//! 
//! ## Components
//! 
//! - [`controller`]: Tauri command handlers for note operations
//! - [`services`]: Note management business logic
//! - [`models`]: Note-related data structures
//! 
//! ## Main Features
//! 
//! - Complete CRUD operations for patient notes
//! - Urgent note flagging
//! - Severity tracking
//! - Integration with patient records
//! - User ownership tracking


pub mod controller;
pub mod models;
pub mod services;
