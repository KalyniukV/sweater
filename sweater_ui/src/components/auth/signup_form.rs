use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::context::auth_context::AuthContext;
use crate::routes::Route;

#[derive(Serialize, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SignupResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[function_component(SignupForm)]
pub fn signup_form() -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let auth_context = use_context::<UseStateHandle<AuthContext>>().unwrap();
    let navigator = use_navigator().unwrap();
    let error_message = use_state(|| Option::<String>::None);

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let auth_context = auth_context.clone();
        let navigator = navigator.clone();
        let error_message = error_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username_clone = username.clone();
            let email_clone = email.clone();
            let password_clone = password.clone();
            let auth_context_clone = auth_context.clone();
            let navigator_clone = navigator.clone();
            let error_message_clone = error_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request_body = SignupRequest {
                    username: username_clone.to_string(),
                    email: email_clone.to_string(),
                    password: password_clone.to_string(),
                };

                match Request::post("http://localhost:3000/signup").json(&request_body).unwrap().send().await {
                    Ok(response) => {
                        match response.json::<SignupResponse>().await {
                            Ok(auth_response) => {
                                let mut new_auth_context = (*auth_context_clone).clone();
                                new_auth_context.logged_in = true;
                                new_auth_context.username = auth_response.username;
                                auth_context_clone.set(new_auth_context);
                                navigator_clone.push(&Route::Home);
                            }
                            Err(_) => {
                                error_message_clone.set(Some("Failed to parse response".to_string()));
                            }
                        }
                    }
                    Err(_) => {
                        error_message_clone.set(Some("Failed to send request".to_string()));
                    }
                }
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <h2>{"Signup"}</h2>
            {if let Some(msg) = &*error_message {
                html! { <p style="color: red;">{msg}</p> }
            } else {
                html! {}
            }}
            <input type="text" placeholder="Username" oninput={on_username_change} />
            <input type="email" placeholder="Email" oninput={on_email_change} />
            <input type="password" placeholder="Password" oninput={on_password_change} />
            <button type="submit">{"Signup"}</button>
        </form>
    }
}