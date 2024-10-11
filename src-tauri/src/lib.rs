
use tauri::Manager;

mod db;
mod users;
mod auth;
mod patients;


// mod notes;
// mod organizations;
// mod patients;
// mod reports;

// mod onnx;


use db::init::{init_db, define_db_on_startup};
use users::controller::get_users;
use auth::login::controller::{login, reset_password};
use auth::signup::controller::signup;
use auth::validate::controller::validate_token;
use patients::controller::{create_patient, delete_patient, get_patients, update_patient};


// use notes::controller::{
//     create_patient_note, delete_patient_note, get_patient_notes, update_patient_note,
// };
// use organizations::controller::get_organizations;
// 
// use reports::controller::{create_report, get_reports, get_report_images};
// use tauri::Manager;

// use onnx::controller::{process_images};

// 


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db_connection = init_db().await.expect("Failed to initialize database");
                
                // Define the database schema
                define_db_on_startup(db_connection.clone())
                    .await
                    .expect("Failed to define database schema");
                
                app.manage(db_connection);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_users, 
            login, 
            reset_password, 
            signup, 
            validate_token, 
            create_patient, 
            delete_patient, 
            get_patients, 
            update_patient
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
