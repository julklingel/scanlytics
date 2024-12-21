use super::models;
use super::services;
use tauri::State;
use scanlytics_db::DbConnection;

/// Retrieves all users from the database.
///
/// This command fetches all user records and returns them as a list of user responses.
/// It requires a database connection state from Tauri.
///
/// # Arguments
///
/// * `db_connection` - Tauri state containing the database connection
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<UserResponse>)` - List of user records
/// * `Err(String)` - Error message if the operation fails


#[tauri::command]
pub async fn get_users(
    db_connection: State<'_, DbConnection>
) -> Result<Vec<models::UserResponse>, String> {
    let db = db_connection.get().lock().await;
    
    let response: Vec<models::UserResponse> = services::get_users_service(&db)
        .await?
        .into_iter()
        .map(|record| models::UserResponse {
            id: record.id,
            name: record.name,
            email: record.email,
            role: record.role,
            organization: record.organization,
            patients: record.patients,
            patient_notes: record.patient_notes,
            statements: record.statements,
            images: record.images,
            reports: record.reports,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
        .collect();
    Ok(response)
}
