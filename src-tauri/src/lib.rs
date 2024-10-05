mod db;
mod notes;
mod organizations;
mod patients;
mod reports;
mod testapi;
mod users;
mod onnx;
mod auth;

use db::init::{define_db_on_startup, init_db};
use notes::controller::{
    create_patient_note, delete_patient_note, get_patient_notes, update_patient_note,
};
use organizations::controller::get_organizations;
use patients::controller::{create_patient, delete_patient, get_patients, update_patient};
use reports::controller::{create_report, get_reports, get_report_images};
use tauri::Manager;
use testapi::controller::{test_db_delete, test_db_read, test_db_write};
use users::controller::get_users;
use onnx::controller::{process_images};
use auth::login::controller::{login, reset_password};
use auth::signup::controller::signup;
use auth::validate::controller::validate_token;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_organizations,
            // Test APIs
            test_db_read,
            test_db_delete,
            test_db_write,
            //Notes APIs
            create_patient_note,
            get_patient_notes,
            update_patient_note,
            delete_patient_note,
            //Patients APIs
            create_patient,
            get_patients,
            update_patient,
            delete_patient,
            // User APIs
            get_users,
            // Report APIs
            create_report,
            get_reports,
            get_report_images,
            process_images,

            // Auth APIs
            login,
            signup,
            reset_password,
            validate_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
