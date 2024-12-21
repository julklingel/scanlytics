//! # Patient Management Module
//! 
//! This module handles all functionality related to patient management including:
//! - Patient creation and registration
//! - Patient information retrieval
//! - Patient record updates
//! - Patient record deletion
//! - Relationship management with doctors
//! 
//! ## Components
//! 
//! - [`controller`]: Tauri command handlers for patient operations
//! - [`services`]: Patient management business logic
//! - [`models`]: Patient-related data structures
//! 
//! ## Main Features
//! 
//! - Complete CRUD operations for patient records
//! - Doctor-patient relationship management
//! - Integration with medical records

pub mod controller;
pub mod models;
pub mod services;
