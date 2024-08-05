use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;



#[tauri::command]
pub async fn get_users(
    db: State<'_, RwLock<Surreal<Client>>>,
) -> Result<Vec<models::UserResponse>, String> {
    let db = db.write().await;
    let response:Vec<models::UserResponse> = services::get_users_service(&db).await?
        .iter()
        .map(|record| models::UserResponse {
            id: record.id.clone(),
            name: record.name.clone(),
            email: record.email.clone(),
            role: record.role.clone(),
            specialization: record.specialization.clone(),
            patients: record.patients.clone(),
            patient_notes: record.patient_notes.clone(),
            created_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        })
        .collect();
    Ok(response)
}
