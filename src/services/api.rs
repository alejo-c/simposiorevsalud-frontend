use crate::types::*;use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::types::*;
pub struct ApiService;

impl ApiService {
    fn get_base_url() -> String {
        if cfg!(debug_assertions) {
            "http://localhost:3000".to_string()
        } else {
            "https://api.simposiorevsalud.univsalud.online".to_string()
        }
    }

    pub async fn register(data: RegisterRequest) -> Result<String, String> {
        let url = format!("{}/api/register", Self::get_base_url());
        
let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn login(data: LoginRequest) -> Result<String, String> {
        let url = format!("{}/api/login", Self::get_base_url());
        
let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn update_user(data: UpdateUserRequest) -> Result<String, String> {
        let url = format!("{}/api/user/update", Self::get_base_url());
        
let response = Request::put(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn generate_horizontal_cert() -> Result<String, String> {
        let url = format!("{}/api/user/horiz-cert", Self::get_base_url());
        
let response = Request::put(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn generate_vertical_cert() -> Result<String, String> {
        let url = format!("{}/api/user/vert-cert", Self::get_base_url());
        
let response = Request::put(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn admin_get_users() -> Result<Vec<User>, String> {
        let url = format!("{}/api/admin/users", Self::get_base_url());
        
let response = Request::post(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("JSON error: {}", e))
        } else {
                .json()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn admin_get_user(id: &str) -> Result<User, String> {
        let url = format!("{}/api/admin/user", Self::get_base_url());
        
let response = Request::post(&url)
            .body(id)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("JSON error: {}", e))
        } else {
                .json()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn admin_update_user(data: AdminUpdateUserRequest) -> Result<String, String> {
        let url = format!("{}/api/admin/update", Self::get_base_url());
        
let response = Request::put(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn delete_user(data: DeleteUserRequest) -> Result<String, String> {
        let url = format!("{}/api/user/delete", Self::get_base_url());
        
let response = Request::delete(&url)
            .header("Content-Type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response error: {}", e))
        } else {
                .text()
                .await
                
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(error_text)
                .text()
                .await
                
        }
    }

    pub async fn logout() -> Result<(), String> {
        let url = format!("{}/api/auth/logout", Self::get_base_url());
        
let response = Request::post(&url)
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
