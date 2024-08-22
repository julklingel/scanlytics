use super::models;
use super::services;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::State;
use tokio::sync::RwLock;



#[tauri::command]
pub async fn get_organizations(
    db: State<'_, RwLock<Surreal<Client>>>,
) -> Result<Vec<models::OrganizationResponse>, String> {
    let db = db.read().await;
    let response:Vec<models::OrganizationResponse> = services::get_organizations_service(&db).await?
        .iter()
        .map(|record| models::OrganizationResponse {
            id: record.id.clone(),
            name: record.name.clone(),
            email: record.email.clone(),
            address: record.address.clone(),
            users: record.users.clone(),
            created_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        })
        .collect();
    Ok(response)
}
