use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub text: String,
    pub created_at: String,
}

impl Notification {
    pub fn new(text: String, user_id: Uuid) -> Self {
        Notification {
            id: Uuid::new_v4(),
            user_id: user_id,
            text: text,
            created_at: Utc::now().naive_utc().to_string(),
        }
    }
}


#[derive(Debug, Serialize, Clone)]
pub struct NotificationWithUserInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub text: String,
    pub created_at: String,
}