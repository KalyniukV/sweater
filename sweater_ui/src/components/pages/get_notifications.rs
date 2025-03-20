use gloo::console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, HtmlElement, HtmlImageElement};
use yew::prelude::*;
use yew::use_state;
use yew::virtual_dom::VNode;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: String,
    pub text: String,
    pub created_at: String,
    pub user_id: String,
    pub username: String
}

#[function_component(GetNotifications)]
pub fn get_notifications() -> Html {
    let notifications_state = use_state(|| Vec::<Notification>::new());
    let error_message_state = use_state(|| Option::<String>::None);

    let page = use_state(|| 1);
    let all_loaded = use_state(|| false);

    let selected_image = use_state(|| None);

    let on_image_click = {
        let selected_image = selected_image.clone();
        Callback::from(move |image_url: String| selected_image.set(Some(image_url)))
    };

    let close_modal = {
        let selected_image = selected_image.clone();
        Callback::from(move |_| selected_image.set(None))
    };

    let fetch_notifications = {
        let notifications_state_clone = notifications_state.clone();
        let error_message_state_clone = error_message_state.clone();
        let page = page.clone();
        let all_loaded_clone = all_loaded.clone();
        Callback::from(move |_| {
            let current_page = *page;

            let notifications_state_clone = notifications_state_clone.clone();
            let error_message_state_clone = error_message_state_clone.clone();
            let page = page.clone();
            let all_loaded_clone = all_loaded_clone.clone();

            spawn_local(async move {
                let url = format!("http://localhost:3000/api/notifications?page={}&per_page=5", current_page);
                let fetched_posts: Result<Vec<Notification>, _> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await;

                match fetched_posts {
                    Ok(new_notifs) => {
                        log!("new_notifs");
                        if new_notifs.is_empty() {
                            all_loaded_clone.set(true);
                        } else {
                            let mut updated_notifs = (*notifications_state_clone).clone();
                            updated_notifs.extend(new_notifs);
                            notifications_state_clone.set(updated_notifs);
                            page.set(current_page + 1);
                        }
                    },
                    Err(err) => error_message_state_clone.set(Some(format!("Error fetching notifications: {:?}", err))),
                }
            });
        })
    };

    {
        let fetch_notifications = fetch_notifications.clone();
        use_effect_with((), move |_| {
            fetch_notifications.emit(()); // Fetch notifications only once on mount
            || () // Cleanup function (empty to prevent re-runs)
        });
    }

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 16px; padding: 20px; font-family: Arial, sans-serif; background-color: #f9f9f9;">

            <div style="display: flex; flex-direction: column; align-items: center; width: 100%; gap: 16px; padding: 20px; background-color: #ffffff; border-radius: 8px; box-shadow: 2px 2px 10px rgba(0, 0, 0, 0.1);">
                <ul style="list-style: none; padding: 0; width: 100%; max-width: 600px;">
                    { for notifications_state.iter().map(|notification| {
                        let wrapped_html = wrap_html_element(create_div(&notification.text), on_image_click.clone());
                        html! {
                            <li style="margin-bottom: 16px; padding: 10px; border-radius: 8px; background: #f5f5f5; box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.1);">
                                <div style="background-color: #ebebeb; padding: 5px; border-radius: 6px;">
                                    <h5 style="margin: 0; font-weight: bold; color: #333;">
                                        { &notification.username } {" "}
                                        <span style="font-size: 12px; color: #777;">{ &notification.created_at }</span>
                                    </h5>
                                </div>
                                <div style="background: white; padding: 12px; border-radius: 6px; margin-top: 6px; box-shadow: 1px 1px 4px rgba(0, 0, 0, 0.08);">
                                    { wrapped_html }
                                </div>
                            </li>
                        }
                    })}
                </ul>

                if !*all_loaded {
                    <button onclick={fetch_notifications.reform(|_| ())}
                        style="background-color: #007bff; color: white; padding: 12px 18px; border: none; border-radius: 6px;
                            font-size: 16px; cursor: pointer; transition: background 0.3s ease-in-out;
                            box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.2);">
                        { "Load More Notifications" }
                    </button>
                }

                // Image Modal (when an image is clicked)
                if let Some(image_url) = (*selected_image).clone() {
                    <div style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0, 0, 0, 0.7); display: flex; justify-content: center; align-items: center;" onclick={close_modal}>
                        <img src={image_url} style="max-width: 90vw; max-height: 90vh; border-radius: 8px;" />
                    </div>
                }
            </div>
        </div>
    }
}

fn wrap_html_element(element: HtmlElement, on_image_click: Callback<String>) -> Html {
    let cloned_element = element.clone();

    // Find all <img> elements inside the given HTML element
    let images = cloned_element.query_selector_all("img").unwrap();

    for i in 0..images.length() {
        if let Some(node) = images.item(i) {
            if let Some(img) = node.dyn_ref::<HtmlImageElement>() {
                let img_src = img.src();
                let on_click = on_image_click.clone();
                let closure = Closure::wrap(Box::new(move || {
                    on_click.emit(img_src.clone());
                }) as Box<dyn Fn()>);

                img.set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget(); // Prevents Rust from dropping it too soon
            }
        }
    }

    VNode::VRef(element.into())
}

fn create_div(text: &str) -> HtmlElement {
    let document: Document = web_sys::window().unwrap().document().unwrap();
    let div: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();
    div.set_inner_html(text);
    div
}