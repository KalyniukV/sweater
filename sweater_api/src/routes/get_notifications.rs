use crate::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_notifications(
    State(state): State<AppState>
) -> Result<impl IntoResponse, String> {

    let notification_store = state.notification_store.read().await;
    let notifications = notification_store.get_all_notifications().await?;

    let resp = Json(notifications);

    Ok((StatusCode::OK, resp))
}

