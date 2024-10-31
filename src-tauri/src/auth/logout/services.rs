use keyring::Entry;



pub async fn logout_service(
) -> Result<String, String> {
    let entry = Entry::new("com.scanlytics.dev", "scanlytics")
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete token: {}", e))?;

    Ok("Logged out".into())
}


    
