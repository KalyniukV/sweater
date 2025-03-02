use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct UserAuthentication {
    pub user_id: Uuid,
    pub password: String,
}

impl UserAuthentication {
    pub fn new(user_id: Uuid, password: String) -> Self {
        UserAuthentication { user_id, password }
    }
}