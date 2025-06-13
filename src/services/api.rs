use crate::types::*;
use gloo_net::http::Request;

pub struct ApiService;

impl ApiService {
    fn get_base_url() -> String {
        if cfg!(debug_assertions) {
            "http://localhost:8000".to_string()
        } else {
            "https://apisimposiorevsalud.univsalud.online".to_string()
        }
    }

    pub async fn login(data: LoginRequest) -> Result<String, String> {
        let url = format!("{}/auth/login", Self::get_base_url());

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            let result: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            Ok(result["token"].as_str().unwrap_or("").to_string())
        } else {
            Err(format!("Login failed: {}", response.status()))
        }
    }

    pub async fn register(data: RegisterRequest) -> Result<String, String> {
        let url = format!("{}/auth/register", Self::get_base_url());

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            let result: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            Ok(result["message"].as_str().unwrap_or("Success").to_string())
        } else {
            Err(format!("Registration failed: {}", response.status()))
        }
    }

    pub async fn update_user(data: UpdateUserRequest) -> Result<String, String> {
        let url = format!("{}/user/update", Self::get_base_url());

        let response = Request::put(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            Ok("Profile updated successfully".to_string())
        } else {
            Err(format!("Update failed: {}", response.status()))
        }
    }

    pub async fn get_users() -> Result<Vec<User>, String> {
        let url = format!("{}/admin/users", Self::get_base_url());

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            let users: Vec<User> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(users)
        } else {
            Err(format!("Failed to get users: {}", response.status()))
        }
    }

    pub async fn admin_get_users() -> Result<Vec<User>, String> {
        Self::get_users().await
    }

    pub async fn admin_get_user(user_id: &str) -> Result<User, String> {
        let url = format!("{}/admin/users/{}", Self::get_base_url(), user_id);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            let user: User = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(user)
        } else {
            Err(format!("Failed to get user: {}", response.status()))
        }
    }

    pub async fn admin_update_user(data: AdminUpdateUserRequest) -> Result<String, String> {
        let url = format!("{}/admin/users/{}", Self::get_base_url(), data.id);

        let response = Request::put(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            Ok("User updated successfully".to_string())
        } else {
            Err(format!("Update failed: {}", response.status()))
        }
    }

    pub async fn delete_user(data: DeleteUserRequest) -> Result<String, String> {
        let url = format!("{}/admin/users/delete", Self::get_base_url());

        let response = Request::delete(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            Ok("User deleted successfully".to_string())
        } else {
            Err(format!("Delete failed: {}", response.status()))
        }
    }

    pub async fn generate_horizontal_cert() -> Result<String, String> {
        let url = format!("{}/certificates/horizontal", Self::get_base_url());

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            Ok("Horizontal certificate generated".to_string())
        } else {
            Err(format!(
                "Certificate generation failed: {}",
                response.status()
            ))
        }
    }

    pub async fn generate_vertical_cert() -> Result<String, String> {
        let url = format!("{}/certificates/vertical", Self::get_base_url());

        let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            Ok("Vertical certificate generated".to_string())
        } else {
            Err(format!(
                "Certificate generation failed: {}",
                response.status()
            ))
        }
    }
}
