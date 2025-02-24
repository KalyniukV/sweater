use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::domain::Notification;

pub async fn create_notification(
    State(state): State<AppState>,
    Json(request): Json<NotificationRequest>
) -> Result<impl IntoResponse, String> {

    let user_id = Uuid::new_v4(); // todo change
    let notification = Notification::new(request.text, user_id);

    let mut notification_store = state.notification_store.write().await;
    let _ = notification_store.create_notification(notification.clone()).await;

    println!("{:#?}", notification);

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct NotificationRequest {
    pub text: String
}
