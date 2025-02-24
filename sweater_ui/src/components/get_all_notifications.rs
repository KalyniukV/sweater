use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::use_state;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    pub url: String,
}

#[function_component(GetNotifications)]
pub fn get_notifications(props: &PostListProps) -> Html {
    let notifications_state = use_state(|| Vec::<Notification>::new());
    let error_message_state = use_state(|| Option::<String>::None);
    let url = props.url.clone(); // Capture the URL

    let cloned_notifications_state_state = notifications_state.clone();
    let cloned_error_message_state = error_message_state.clone();

    use_effect(move || {
        spawn_local(async move {
            let fetched_posts: Result<Vec<Notification>, _> = Request::get(&url)
                .send()
                .await
                .unwrap()
                .json()
                .await;

            match fetched_posts {
                Ok(p) => cloned_notifications_state_state.set(p),
                Err(err) => cloned_error_message_state.set(Some(format!("Error fetching notifications: {:?}", err))),
            }
        });
    });

    html! {
        <div>
            <h2>{"Notifications"}</h2>
            {
                if let Some(msg) = &*error_message_state {
                    html! { <p style="color: red;">{msg}</p> }
                } else {
                    html!{}
                }
            }
            <ul>
                {
                    (*notifications_state).iter().map(|notification| {
                        html! { <li>{format!("{}: {}", notification.id, notification.text)}</li> }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}