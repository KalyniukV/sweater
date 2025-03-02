use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub username: Option<String>,
}