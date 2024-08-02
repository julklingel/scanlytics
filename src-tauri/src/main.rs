// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;
mod testapi;
mod notes;
mod patients;

use tauri::Manager;
use db::init::init_db;
use testapi::controller::{test_db_write, test_db_read, test_db_delete};
use notes::controller::{create_patient_note, get_patient_notes, update_patient_note, delete_patient_note};	
use patients::controller::{create_patient, get_patients, update_patient, delete_patient};	




fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db().await.expect("Failed to initialize database");
                app.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![

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
            delete_patient

            
            
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}