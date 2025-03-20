use crate::app_state::AppState;
use axum::extract::{State, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

pub async fn get_notifications(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, String> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(5);
    let offset = (page - 1) * per_page;

    let notification_store = state.notification_store.read().await;
    let notifications = notification_store.get_notifications(per_page, offset).await?;

    let resp = Json(notifications);

    Ok((StatusCode::OK, resp))
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}