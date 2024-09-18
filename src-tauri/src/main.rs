// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
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


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db().await.expect("Failed to initialize database");

                {
                    let db_guard = db.read().await;
                    define_db_on_startup(&*db_guard)
                        .await
                        .expect("Failed to define database structure");
                }

                app.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Organizations APIs
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
            reset_password


            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

