use gloo::console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, HtmlElement};
use yew::prelude::*;
use yew::use_state;
use yew::virtual_dom::VNode;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub text: String,
    pub created_at: String,
}

#[function_component(GetNotifications)]
pub fn get_notifications() -> Html {
    let notifications_state = use_state(|| Vec::<Notification>::new());
    let error_message_state = use_state(|| Option::<String>::None);

    let fetch_notifications = {
        let notifications_state_clone = notifications_state.clone();
        let error_message_state_clone = error_message_state.clone();
        Callback::from(move |_| {
            let notifications_state_clone = notifications_state_clone.clone();
            let error_message_state_clone = error_message_state_clone.clone();
            spawn_local(async move {
                log!("spawn notifications!!!!");
                let fetched_posts: Result<Vec<Notification>, _> = Request::get("http://localhost:3000/api/notifications")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await;

                match fetched_posts {
                    Ok(notif) => notifications_state_clone.set(notif),
                    Err(err) => error_message_state_clone.set(Some(format!("Error fetching notifications: {:?}", err))),
                }
            });
        })
    };

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 16px; padding: 20px; font-family: Arial, sans-serif; background-color: #f9f9f9;">
            <button onclick={fetch_notifications}
                style="background-color: #007bff; color: white; padding: 10px 16px; border: none; border-radius: 6px;
                       font-size: 16px; cursor: pointer; transition: background 0.3s; box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.2);">
                {"Fetch Notifications"}
            </button>

            <ul style="list-style: none; padding: 0; width: 100%;">
                { for notifications_state.iter().map(|notification| html! {
                    <li style="background: white; padding: 12px; border-radius: 8px; margin-bottom: 10px; box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.1); display: flex; align-items: center; gap: 10px;">
                        <p style="margin: 0; font-size: 14px; color: #333;">{wrap_html_element(get_div(&notification.text))}</p>
                    </li>
                }) }
            </ul>
        </div>
    }
}

fn wrap_html_element(element: HtmlElement) -> Html {
    VNode::VRef(element.into())
}

fn get_div(text: &str) -> HtmlElement {
    let document: Document = web_sys::window().unwrap().document().unwrap();
    let div: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();

    div.set_inner_html(text);

    div
}