use std::sync::Arc;
use tokio::sync::RwLock;
use crate::data_store::NotificationStore;

pub type NotificationStoreType = Arc<RwLock<dyn NotificationStore>>;

#[derive(Clone)]
pub struct AppState {
    pub notification_store: NotificationStoreType
}

impl AppState {
    pub fn new(notification_store: NotificationStoreType) -> Self {
        AppState {
            notification_store
        }
    }
}