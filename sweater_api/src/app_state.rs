use std::sync::Arc;
use tokio::sync::RwLock;
use crate::data_store::{AuthenticationStore, NotificationStore, UserStore};

pub type NotificationStoreType = Arc<RwLock<dyn NotificationStore>>;
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;
pub type AuthenticationStoreType = Arc<RwLock<dyn AuthenticationStore>>;

#[derive(Clone)]
pub struct AppState {
    pub notification_store: NotificationStoreType,
    pub user_store: UserStoreType,
    pub authentication_store: AuthenticationStoreType,
}

impl AppState {
    pub fn new(notification_store: NotificationStoreType,
               user_store: UserStoreType,
               authentication_store: AuthenticationStoreType
    ) -> Self {
        AppState {
            notification_store,
            user_store,
            authentication_store,
        }
    }
}