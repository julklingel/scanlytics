
#[macro_use]
mod commands;

mod users;
mod auth;
mod patients;
mod notes;
mod reports;
mod image_analysis;




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                if let Err(e) = scanlytics_db::setup_database(app).await {
                    eprintln!("Failed to setup database: {:?}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(get_commands!()) 
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
