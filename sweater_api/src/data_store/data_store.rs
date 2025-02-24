use crate::domain::Notification;

#[async_trait::async_trait]
pub trait NotificationStore: Send + Sync {
    async fn create_notification(&mut self, notification: Notification) -> Result<(), String>;
    async fn get_all_notifications(&self) -> Result<Vec<Notification>, String>;
}