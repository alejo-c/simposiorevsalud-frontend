use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ApiService {
    base_url: String,
}

impl ApiService {
    pub fn new() -> Self {
        let base_url = if cfg!(debug_assertions) {
            "http://localhost:8000".to_string()
        } else {
            "https://api.simposiorevsalud.univsalud.online".to_string()
        };

        Self { base_url }
    }

    pub async fn register_user(&self, form_data: RegisterForm) -> Result<ApiResponse, String> {
        let url = format!("{}/register", self.base_url);

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&form_data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.ok() {
            let result: ApiResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(result)
        } else {
            Err(format!("API error: {}", response.status()))
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, String> {
        let url = format!("{}/users", self.base_url);

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
            Err(format!("API error: {}", response.status()))
        }
    }

    // Add other API methods as needed
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub institution: Option<String>,
    pub phone: Option<String>,
    // Add other fields from your original form
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    // Add other user fields
}
