
#[macro_export]
macro_rules! get_commands {
    () => {
        tauri::generate_handler![
            // Auth
            crate::users::controller::get_users,
            crate::auth::login::controller::login,
            crate::auth::signup::controller::signup,
            crate::auth::logout::controller::logout,
            crate::auth::validate::controller::validate_token,
            // Patients
            crate::patients::controller::create_patient,
            crate::patients::controller::delete_patient,
            crate::patients::controller::get_patients,
            crate::patients::controller::update_patient,
            // Notes
            crate::notes::controller::create_patient_note,
            crate::notes::controller::delete_patient_note,
            crate::notes::controller::get_patient_notes,
            crate::notes::controller::update_patient_note,
            // Reports
            crate::reports::controller::create_report,
            crate::reports::controller::get_reports,
            crate::reports::controller::get_report_images,
            // Image Analysis
            crate::image_analysis::image_processing::controller::process_images
        ]
    };
}