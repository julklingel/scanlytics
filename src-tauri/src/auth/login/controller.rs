use super::models;
use super::services;


#[tauri::command]
pub async fn login(

    login_data: String,
) -> Result<models::LoginResponse, String> {
    let response = services::login_service( login_data).await?;
    Ok(response)
}

#[tauri::command]
pub async fn reset_password(reset_data: String) -> Result<models::ResetPasswordResponse, String> {
    let response = services::reset_password_service(reset_data).await?;
    Ok(response)
}
