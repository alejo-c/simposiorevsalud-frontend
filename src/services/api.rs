use crate::services::auth::AuthService;
use crate::types::*;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Response};

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

        log::info!("Making request to: {}", url);

        let request = Request::post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .credentials(web_sys::RequestCredentials::Omit);

        let request = match request.json(&data) {
            Ok(req) => req,
            Err(e) => {
                log::error!("JSON serialization error: {:?}", e);
                return Err(format!("Request serialization error: {}", e));
            }
        };

        log::info!("Sending request...");

        let response = match request.send().await {
            Ok(resp) => resp,
            Err(e) => {
                log::error!("Network error details: {:?}", e);
                return Err(format!("Connection failed: {}. Please check if the server is running and CORS is properly configured.", e));
            }
        };

        log::info!("Response status: {}", response.status());

        if response.ok() {
            match response.text().await {
                Ok(text) => {
                    log::info!("Success response: {}", text);
                    Ok(text)
                }
                Err(e) => {
                    log::error!("Response reading error: {:?}", e);
                    Err(format!("Response reading error: {}", e))
                }
            }
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| format!("HTTP {} error", status));

            log::error!("HTTP error {}: {}", status, error_text);
            Err(format!("Server error ({}): {}", status, error_text))
        }
    }

    pub async fn login(data: LoginRequest) -> Result<String, String> {
        let url = format!("{}/login", Self::get_base_url());

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .credentials(web_sys::RequestCredentials::Omit)
            .json(&data)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}. Please check server connectivity.", e))?;

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

    // Debug functions to test API connectivity
    pub async fn test_api_connection() -> Result<String, String> {
        let window = web_sys::window().unwrap();

        // Test simple GET request to check if server is reachable
        let test_url = "https://apisimposiorevsalud.univsalud.online/";

        let mut opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = web_sys::Request::new_with_str_and_init(test_url, &opts)
            .map_err(|e| format!("Failed to create request: {:?}", e))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| format!("Fetch failed: {:?}", e))?;

        let resp: Response = resp_value.dyn_into().unwrap();

        Ok(format!("Server responded with status: {}", resp.status()))
    }

    pub async fn test_options_request() -> Result<String, String> {
        let window = web_sys::window().unwrap();

        let test_url = "https://apisimposiorevsalud.univsalud.online/register";

        let mut opts = RequestInit::new();
        opts.set_method("OPTIONS");
        opts.set_mode(RequestMode::Cors);

        let headers = web_sys::Headers::new().unwrap();
        headers
            .set("Origin", "https://simposiorevsalud.univsalud.online")
            .unwrap();
        headers
            .set("Access-Control-Request-Method", "POST")
            .unwrap();
        headers
            .set("Access-Control-Request-Headers", "content-type")
            .unwrap();
        opts.set_headers(&headers);

        let request = web_sys::Request::new_with_str_and_init(test_url, &opts)
            .map_err(|e| format!("Failed to create OPTIONS request: {:?}", e))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| format!("OPTIONS request failed: {:?}", e))?;

        let resp: Response = resp_value.dyn_into().unwrap();

        Ok(format!("OPTIONS request status: {}", resp.status()))
    }
}
