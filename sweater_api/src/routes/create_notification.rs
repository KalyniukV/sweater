use crate::app_state::AppState;
use crate::domain::Notification;
use crate::utils::{get_images, save_image, DATA_STORAGE_PATH};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use regex::Regex;
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

    let images_as_base = get_images(&request_body);

    match &images_as_base {
        Some(pictures) => {
            let images_path = save_image(&user_id, pictures);
            request_body = replace_image(request_body, pictures, &images_path);
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

fn replace_image(text: String, pictures_as_base: &Vec<String>, path: &Vec<String>) -> String {
    let mut result = text;

    for i in 0..pictures_as_base.len() {
        let host_path = path[i].as_str().replace(DATA_STORAGE_PATH.as_str(), "http://localhost:3000/images/");

        replace_picture_as_base_to_host_path(&mut result, pictures_as_base[i].as_str(), &host_path);
        remove_prefix_format(&mut result);
    }

    result
}

fn replace_picture_as_base_to_host_path(text: &mut String, pictures_as_base: &str, host_path: &str) {
    *text = text.replace(pictures_as_base, &host_path);
}

fn remove_prefix_format(text: &mut String) {
    match extract_format(&text) {
        Some(format) => {
            let prefix_format = format!("data:image/{};base64,", format);
            *text = text.replace(prefix_format.as_str(), "");
        },
        None => {println!("Please check your image")}
    }
}

fn extract_format(picture_format: &str) -> Option<String> {
    let re = Regex::new(r"data:image/([^;]+);base64").unwrap();
    re.captures(picture_format)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}