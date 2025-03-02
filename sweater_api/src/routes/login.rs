use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::app_state::AppState;


pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>
) -> Result<impl IntoResponse, String> {

    let user_store = state.user_store.read().await;
    let user = user_store.get_user(&request.email).await?;

    let authentication_store = state.authentication_store.read().await;
    let user_credentials = authentication_store.get_authentication(&user.id).await?;

    if user_credentials.password != request.password {
        return Err("wrong password".to_string());
    }

    let resp = Json(user);

    Ok((StatusCode::OK, resp))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}