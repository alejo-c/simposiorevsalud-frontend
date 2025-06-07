use gloo_storage::{LocalStorage, Storage};

pub struct AuthService;

impl AuthService {
    const JWT_KEY: &'static str = "jwt";

    pub fn get_token() -> Option<String> {
        LocalStorage::get(Self::JWT_KEY).ok()
    }

    pub fn set_token(token: String) {
        let _ = LocalStorage::set(Self::JWT_KEY, token);
    }

    pub fn remove_token() {
        LocalStorage::delete(Self::JWT_KEY);
    }

    pub fn is_authenticated() -> bool {
        Self::get_token().is_some()
    }

    pub async fn logout() {
        Self::remove_token();
        if let Err(e) = crate::services::api::ApiService::logout().await {
            log::warn!("Logout API call failed: {}", e);
        }
    }
}
