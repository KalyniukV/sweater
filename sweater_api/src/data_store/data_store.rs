use uuid::Uuid;
use crate::domain::{Notification, User, UserAuthentication};

#[async_trait::async_trait]
pub trait NotificationStore: Send + Sync {
    async fn create_notification(&mut self, notification: &Notification) -> Result<(), String>;
    async fn get_all_notifications(&self) -> Result<Vec<Notification>, String>;
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn create_user(&mut self, user: &User) -> Result<(), String>;
    async fn get_user(&self, email: &str) -> Result<User, String>;
}

#[async_trait::async_trait]
pub trait AuthenticationStore: Send + Sync {
    async fn create_authentication(&mut self, user_id: &Uuid, password: &str) -> Result<(), String>;
    async fn get_authentication(&self, user_id: &Uuid) -> Result<UserAuthentication, String>;

}