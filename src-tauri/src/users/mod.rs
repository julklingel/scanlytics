//! # User Management Module
//! 
//! Handles all user-related operations including creation, retrieval,
//! and management of user accounts.
//! 
//! ## Components
//! 
//! - [`controller`]: Tauri command handlers for user operations
//! - [`services`]: User management business logic
//! - [`models`]: User-related data structures
//! 
//! ## Usage
//! 
//! This module is typically accessed through Tauri commands from the frontend.
//! See individual controller documentation for available commands.

pub mod controller;
pub mod models;
pub mod services;
