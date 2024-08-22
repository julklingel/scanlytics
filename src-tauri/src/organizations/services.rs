use super::models::OrganizationResponse;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;



pub async fn get_organizations_service(db: &Surreal<Client>) -> Result<Vec<OrganizationResponse>, String> {
    let records: Vec<OrganizationResponse> = db.select("Organization").await.map_err(|e| e.to_string())?;
    Ok(records)
}
