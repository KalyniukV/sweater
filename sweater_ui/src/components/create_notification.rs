use gloo::console::log;
use gloo_net::http::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    text: String
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: String,
}

#[function_component(CreateNotification)]
pub fn post_form(props: &Props) -> Html {
    let body_state = use_state(|| String::new());
    let response_state = use_state(|| String::new());

    let cloned_body_state = body_state.clone();
    let on_body_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        cloned_body_state.set(input.value());
    });

    let on_submit = {
        let url = props.url.clone();
        let body = body_state.clone();
        // let response = response.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Prevent form from reloading the page

            let post_data = PostData {
                text: (*body).clone(),
            };

            let url = url.clone();
            // let response = response.clone();

            spawn_local(async move {
                let result = RequestBuilder::new(&url)
                    .method(Method::POST)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&post_data).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match result {
                    Ok(resp) => {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_else(|_| "Failed to get response text".to_string());
                        log!(format!("Status: {}\nResponse: {}", status, text));
                    }
                    Err(err) => {
                        log!(format!("Error: {:?}", err));
                    }
                }
            });
        })
    };


    html! {
        <div>
            <h1>{"Create Notification"}</h1>
            <form onsubmit={on_submit}>
                <textarea id="notification" value={(*body_state).clone()} onchange={on_body_change} /><br/><br/>

                <button type="submit">{"Send"}</button>
            </form>

            <pre>{(*response_state).clone()}</pre>
        </div>
    }
}