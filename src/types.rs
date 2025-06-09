use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub full_name: String,
    pub identification: String,
    pub password: String,
    pub role: String,
    pub presentation: String,
    pub attendance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub password: String,
    pub attendance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUpdateUserRequest {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub identification: String,
    pub password: String,
    pub role: String,
    pub presentation: String,
    pub attendance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub email: String,
    pub full_name: String,
    pub identification: String,
    pub password: String,
    pub role: String,
    pub presentation: String,
    pub attendance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub identification: String,
    pub role: UserRole,
    pub presentation: Option<String>,
    pub attendance: String,
    pub cert_generated: CertificateStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRole {
    Simple(String),
    Speaker { speaker: SpeakerInfo },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerInfo {
    pub presentation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateStatus {
    pub horizontal: bool,
    pub vertical: bool,
}
