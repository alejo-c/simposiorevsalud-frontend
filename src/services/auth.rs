use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, window};

#[wasm_bindgen]
extern "C" {
    // Netlify Identity Widget bindings
    #[wasm_bindgen(js_namespace = netlifyIdentity)]
    fn init();

    #[wasm_bindgen(js_namespace = netlifyIdentity)]
    fn open();

    #[wasm_bindgen(js_namespace = netlifyIdentity)]
    fn close();

    #[wasm_bindgen(js_namespace = netlifyIdentity)]
    fn currentUser() -> JsValue;

    #[wasm_bindgen(js_namespace = netlifyIdentity)]
    fn logout();

    #[wasm_bindgen(js_namespace = netlifyIdentity, js_name = on)]
    fn on(event: &str, callback: &js_sys::Function);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetlifyUser {
    pub id: String,
    pub email: String,
    pub user_metadata: UserMetadata,
    pub app_metadata: AppMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserMetadata {
    pub full_name: Option<String>,
    pub identification: Option<String>,
    pub role: Option<String>,
    pub attendance: Option<String>,
    pub certificates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppMetadata {
    pub roles: Option<Vec<String>>,
}

pub struct AuthService;

impl AuthService {
    pub fn init() {
        init();

        // Set up event listeners
        let login_callback = Closure::wrap(Box::new(|user: JsValue| {
            log::info!("User logged in: {:?}", user);
            Self::store_user_locally(&user);
        }) as Box<dyn Fn(JsValue)>);

        let logout_callback = Closure::wrap(Box::new(|| {
            log::info!("User logged out");
            Self::clear_user_locally();
        }) as Box<dyn Fn()>);

        on("login", login_callback.as_ref().unchecked_ref());
        on("logout", logout_callback.as_ref().unchecked_ref());

        // Don't forget these or memory will leak
        login_callback.forget();
        logout_callback.forget();
    }

    pub fn open_auth() {
        open();
    }

    pub fn logout_user() {
        logout();
        Self::clear_user_locally();
    }

    pub fn get_current_user() -> Option<NetlifyUser> {
        let user_js = currentUser();

        if user_js.is_null() || user_js.is_undefined() {
            // Try to get from local storage
            return Self::get_user_from_storage();
        }

        // Convert JS user to Rust struct
        match serde_wasm_bindgen::from_value::<NetlifyUser>(user_js) {
            Ok(user) => {
                Self::store_user_locally(&serde_wasm_bindgen::to_value(&user).unwrap());
                Some(user)
            }
            Err(e) => {
                log::error!("Failed to parse user: {:?}", e);
                None
            }
        }
    }

    pub fn is_authenticated() -> bool {
        Self::get_current_user().is_some()
    }

    pub fn is_admin() -> bool {
        if let Some(user) = Self::get_current_user() {
            // Check app_metadata roles
            if let Some(roles) = user.app_metadata.roles {
                return roles.contains(&"admin".to_string());
            }

            // Fallback to user_metadata role
            if let Some(role) = user.user_metadata.role {
                return role == "admin";
            }
        }
        false
    }

    // Update user metadata (for registration completion)
    pub async fn update_user_metadata(metadata: UserMetadata) -> Result<(), String> {
        let user = Self::get_current_user().ok_or("No user logged in")?;

        // In a real implementation, you'd call Netlify's API to update user metadata
        // For now, we'll store it locally and send via EmailJS
        let window = window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let updated_user = NetlifyUser {
            user_metadata: metadata,
            ..user
        };

        let user_json = serde_json::to_string(&updated_user)
            .map_err(|e| format!("Failed to serialize user: {}", e))?;

        storage
            .set_item("netlify_user_extended", &user_json)
            .map_err(|e| format!("Failed to store user: {:?}", e))?;

        Ok(())
    }

    fn store_user_locally(user_js: &JsValue) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(user_str) = js_sys::JSON::stringify(user_js) {
                    let _ = storage.set_item("netlify_user", &user_str.as_string().unwrap());
                }
            }
        }
    }

    fn clear_user_locally() {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("netlify_user");
                let _ = storage.remove_item("netlify_user_extended");
            }
        }
    }

    fn get_user_from_storage() -> Option<NetlifyUser> {
        let window = window()?;
        let storage = window.local_storage().ok()??;

        // Try extended user first
        if let Ok(Some(user_json)) = storage.get_item("netlify_user_extended") {
            if let Ok(user) = serde_json::from_str::<NetlifyUser>(&user_json) {
                return Some(user);
            }
        }

        // Fallback to basic netlify user
        if let Ok(Some(user_str)) = storage.get_item("netlify_user") {
            if let Ok(user_js) = js_sys::JSON::parse(&user_str) {
                if let Ok(user) = serde_wasm_bindgen::from_value::<NetlifyUser>(user_js) {
                    return Some(user);
                }
            }
        }

        None
    }
}
