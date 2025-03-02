use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::app_state::AppState;
use crate::domain::User;

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> Result<impl IntoResponse, String> {

    let user = User::new(request.username, request.email);

    let mut user_store = state.user_store.write().await;
    user_store.create_user(&user).await?;

    let mut authentication_store = state.authentication_store.write().await;
    let _ = authentication_store.create_authentication(&user.id, &request.password).await?;

    let resp = Json(user);

    Ok((StatusCode::CREATED, resp))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}