//! # Logout Module
//! 
//! Handles user session termination and cleanup, including:
//! - Token removal from system keyring
//! - Session cleanup
//! - Secure logout procedures
//! 
//! ## Features
//! 
//! - Secure token removal
//! - Session state management
//! - Error handling for cleanup operations
//! 

pub mod controller;
pub mod services;
pub mod models;
