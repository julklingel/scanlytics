
#[derive(Debug)]
pub enum LogoutError {
    KeyringAccess(String),
    KeyringDelete(String),
}

impl std::fmt::Display for LogoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogoutError::KeyringAccess(detail) => write!(f, "Keyring access error: {}", detail),
            LogoutError::KeyringDelete(detail) => write!(f, "Keyring delete error: {}", detail),
        }
    }
}