use crate::services::auth::AuthService;
use crate::types::*;
use gloo_net::http::Request;

pub struct ApiService;

impl ApiService {
    fn get_base_url() -> String {
        if cfg!(debug_assertions) {
            "http://localhost:3000".to_string()
        } else {
            "https://apisimposiorevsalud.univsalud.online".to_string()
        }
    }

    fn add_auth_header(request: Request) -> Request {
        if let Some(token) = AuthService::get_token() {
            request.header("Authorization", &format!("Bearer {}", token))
        } else {
            request
        }
    }

    pub async fn register(data: RegisterRequest) -> Result<String, String> {
        let url = format!("{}/register", Self::get_base_url());

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn login(data: LoginRequest) -> Result<String, String> {
        let url = format!("{}/login", Self::get_base_url());

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn update_user(data: UpdateUserRequest) -> Result<String, String> {
        let url = format!("{}/user/update", Self::get_base_url());

        let response = Self::add_auth_header(Request::put(&url))
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn generate_horizontal_cert() -> Result<String, String> {
        let url = format!("{}/user/horiz-cert", Self::get_base_url());

        let response = Self::add_auth_header(Request::put(&url))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn generate_vertical_cert() -> Result<String, String> {
        let url = format!("{}/user/vert-cert", Self::get_base_url());

        let response = Self::add_auth_header(Request::put(&url))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn admin_get_users() -> Result<Vec<User>, String> {
        let url = format!("{}/admin/users", Self::get_base_url());

        let response = Self::add_auth_header(Request::post(&url))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| format!("JSON error: {}", e))
        } else if response.status() == 401 {
            Err("No autorizado. Por favor inicie sesión nuevamente.".to_string())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn admin_get_user(id: &str) -> Result<User, String> {
        let url = format!("{}/admin/user", Self::get_base_url());

        let response = Self::add_auth_header(Request::post(&url))
            .body(id)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| format!("JSON error: {}", e))
        } else if response.status() == 401 {
            Err("No autorizado. Por favor inicie sesión nuevamente.".to_string())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn admin_update_user(data: AdminUpdateUserRequest) -> Result<String, String> {
        let url = format!("{}/admin/update", Self::get_base_url());

        let response = Self::add_auth_header(Request::put(&url))
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else if response.status() == 401 {
            Err("No autorizado. Por favor inicie sesión nuevamente.".to_string())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn delete_user(data: DeleteUserRequest) -> Result<String, String> {
        let url = format!("{}/user/delete", Self::get_base_url());

        let response = Self::add_auth_header(Request::delete(&url))
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .text()
                .await
                .map_err(|e| format!("Response error: {}", e))
        } else if response.status() == 401 {
            Err("No autorizado. Por favor inicie sesión nuevamente.".to_string())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
        }
    }

    pub async fn logout() -> Result<(), String> {
        let url = format!("{}/logout", Self::get_base_url());

        let response = Self::add_auth_header(Request::post(&url))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            Ok(())
        } else {
            Err("Logout failed".to_string())
        }
    }
}
