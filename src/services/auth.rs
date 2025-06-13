use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "auth_token";
const USER_KEY: &str = "current_user";

pub struct AuthService;

impl AuthService {
    pub fn set_token(token: String) {
        let _ = LocalStorage::set(TOKEN_KEY, token);
    }

    pub fn get_token() -> Option<String> {
        LocalStorage::get(TOKEN_KEY).ok()
    }

    pub fn remove_token() {
        LocalStorage::delete(TOKEN_KEY);
    }

    pub fn is_authenticated() -> bool {
        Self::get_token().is_some()
    }

    pub fn logout() {
        Self::remove_token();
        LocalStorage::delete(USER_KEY);
    }

    // You can add methods to store/retrieve user info if needed
    pub fn set_user_info(user: serde_json::Value) {
        let _ = LocalStorage::set(USER_KEY, user);
    }

    pub fn get_user_info() -> Option<serde_json::Value> {
        LocalStorage::get(USER_KEY).ok()
    }
}
