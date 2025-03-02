use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
            email,
        }
    }
}