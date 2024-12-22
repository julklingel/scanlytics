//! # Authentication Module
//! 
//! This module handles all authentication-related functionality including:
//! - User login and token management
//! - Signup process
//! - Token validation
//! - Logout handling
//! 
//! ## Components
//! 
//! - [`login`]: User login and token generation
//! - [`signup`]: New user registration
//! - [`validate`]: Token validation and verification
//! - [`logout`]: Session termination
//! 


pub mod signup;
pub mod login;
pub mod validate;
pub mod logout;
