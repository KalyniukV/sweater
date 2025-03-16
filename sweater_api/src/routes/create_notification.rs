use crate::app_state::AppState;
use crate::domain::Notification;
use crate::utils::{getImages, save_image, DATA_STORAGE_PATH};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

pub async fn create_notification(
    State(state): State<AppState>,
    Json(request): Json<NotificationRequest>
) -> Result<impl IntoResponse, String> {

    let user_id = request.user_id;
    let mut request_body = request.text;

    if request_body.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }

    // println!("\n\nbefore:\n{}", request_body);

    let images_as_base = getImages(&request_body);

    match &images_as_base {
        Some(pictures) => {
            let images_path = save_image(&user_id, pictures);
            request_body = replace_image_base_by_host_path(request_body, pictures, &images_path);
        }
        None => println!("Please check your image"),
    }

    println!("\n\nafter:\n{}", request_body);

    let user_id = Uuid::parse_str(&user_id).unwrap(); //todo handle error
    let notification = Notification::new(request_body, user_id);

    let mut notification_store = state.notification_store.write().await;
    let _ = notification_store.create_notification(&notification.clone()).await;

    println!("{:#?}", notification);

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct NotificationRequest {
    pub user_id: String,
    pub text: String
}

fn replace_image_base_by_host_path(text: String, pictures_as_base: &Vec<String>, path: &Vec<String>) -> String {
    let mut result = text;
    let prefix_jpeg = "data:image/jpeg;base64,"; // todo remove
    let prefix_png = "data:image/png;base64,"; // todo remove
    for i in 0..pictures_as_base.len() {
        let host_path = path[i].as_str().replace(DATA_STORAGE_PATH.as_str(), "http://localhost:3000/images/");

        result = result.replace(pictures_as_base[i].as_str(), &host_path);
        result = result.replace(prefix_jpeg, "");
        result = result.replace(prefix_png, "");
    }

    result
}