
use tauri::Manager;
mod users;
mod auth;
mod patients;
mod notes;
mod reports;
mod image_analysis;


use users::controller::get_users;
use auth::login::controller::login;
use auth::signup::controller::signup;
use auth::logout::controller::logout;
use auth::validate::controller::validate_token;
use patients::controller::{create_patient, delete_patient, get_patients, update_patient};
use notes::controller::{create_patient_note, delete_patient_note, get_patient_notes, update_patient_note};
use reports::controller::{create_report, get_reports, get_report_images};
use image_analysis::image_processing::controller::process_images;
use scanlytics_db::{init_db, define_db_on_startup};





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                match init_db(&app.app_handle(), true).await {
                    Ok(db_connection) => {
                        match define_db_on_startup(db_connection.clone()).await {
                            Ok(_) => {
                                app.manage(db_connection);
                                println!("Database setup completed successfully");
                            },
                            Err(e) => eprintln!("Failed to define database schema: {:?}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to initialize database: {:?}", e),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_users, 
            login,
            logout,
            signup, 
            validate_token, 
            create_patient, 
            delete_patient, 
            get_patients, 
            update_patient,
            create_patient_note,
            delete_patient_note,
            get_patient_notes,
            update_patient_note,
            create_report,
            get_reports,
            get_report_images,
            process_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
