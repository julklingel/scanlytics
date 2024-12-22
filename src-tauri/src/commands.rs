/// # Command Registration Macro
///
/// This macro generates a Tauri command handler that registers all available
/// application commands. It provides a centralized way to manage and register
/// all command handlers across different modules.
///
/// ## Available Commands
///
/// ### Authentication Commands
/// - `login`: User authentication
/// - `signup`: New user registration
/// - `logout`: User session termination
/// - `validate_token`: Token validation
///
/// ### User Management
/// - `get_users`: Retrieve user information
///
/// ### Patient Management
/// - `create_patient`: Create new patient records
/// - `delete_patient`: Remove patient records
/// - `get_patients`: Retrieve patient information
/// - `update_patient`: Modify patient records
///
/// ### Patient Notes
/// - `create_patient_note`: Create medical notes
/// - `delete_patient_note`: Remove medical notes
/// - `get_patient_notes`: Retrieve medical notes
/// - `update_patient_note`: Modify medical notes
///
/// ### Medical Reports
/// - `create_report`: Generate medical reports
/// - `get_reports`: Retrieve report information
/// - `get_report_images`: Access report images
///
/// ### Image Analysis
/// - `process_images`: Perform medical image processing

/// ## Implementation Details
///
/// The macro expands to a `tauri::generate_handler!` macro call that includes
/// all available command handlers. Each handler is referenced using the full
/// path to ensure proper resolution.


#[macro_export]
macro_rules! get_commands {
    () => {
        tauri::generate_handler![
            // Auth
            $crate::users::controller::get_users,
            $crate::auth::login::controller::login,
            $crate::auth::signup::controller::signup,
            $crate::auth::logout::controller::logout,
            $crate::auth::validate::controller::validate_token,
            // Patients
            $crate::patients::controller::create_patient,
            $crate::patients::controller::delete_patient,
            $crate::patients::controller::get_patients,
            $crate::patients::controller::update_patient,
            // Notes
            $crate::notes::controller::create_patient_note,
            $crate::notes::controller::delete_patient_note,
            $crate::notes::controller::get_patient_notes,
            $crate::notes::controller::update_patient_note,
            // Reports
            $crate::reports::controller::create_report,
            $crate::reports::controller::get_reports,
            $crate::reports::controller::get_report_images,
            // Image Analysis
            $crate::image_analysis::image_processing::controller::process_images
        ]
    };
}