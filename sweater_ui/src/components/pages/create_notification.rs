use gloo::console::log;
use gloo_file::futures::read_as_data_url;
use gloo_file::File;
use gloo_net::http::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlDivElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    user_id: String,
    text: String
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_id: String,
}

#[function_component(CreateNotification)]
pub fn create_notification(props: &Props) -> Html {
    let content: UseStateHandle<String> = use_state(String::new);
    let editor_ref = use_node_ref();
    let user_id = props.user_id.clone();

    let oninput = {
        let content = content.clone();
        let editor_ref_clone = editor_ref.clone();

        Callback::from(move |_| {
            if let Some(editor) = editor_ref_clone.cast::<HtmlDivElement>() {
                log!("Input text was changed: ", editor.inner_html());
                content.set(editor.inner_html());
            }
        })
    };

    let onchange_file = {
        let content = content.clone();
        let editor_ref_clone = editor_ref.clone();

        Callback::from(move |e: Event| {
            let content = content.clone();
            let target: HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = target.files() {
                if let Some(file_blob) = file_list.get(0) {
                    let file_blob = File::from(file_blob);
                    let editor_ref_clone = editor_ref_clone.clone();

                    spawn_local(async move {
                        match read_as_data_url(&file_blob).await {
                            Ok(data_url) => {
                                if let Some(editor) = editor_ref_clone.cast::<HtmlInputElement>() {
                                    let img_tag = format!("<img src=\"{}\" style=\"max-width: 100px; max-height: 100px;\" />", data_url);
                                    // let img_tag = format!("<img src=\"{}\"/>", data_url);
                                    editor.set_inner_html(&format!("{}{}", editor.inner_html(), img_tag));
                                    content.set(editor.inner_html());
                                }
                            }
                            Err(err) => {
                                web_sys::console::log_1(&format!("Error reading file: {:?}", err).into());
                            }
                        }
                    });
                }
            }
        })
    };

    let on_submit = {
        let editor_ref_clone = editor_ref.clone();
        let user_id = user_id.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); // Prevent form from reloading the page

            let content_clone = content.clone();

            let post_data = PostData {
                user_id: user_id.clone(),
                text: (*content_clone).clone(),
            };

            log!("send: ", &post_data.text);

            let editor_ref_clone = editor_ref_clone.clone();

            spawn_local(async move {
                let content_clone = content_clone.clone();

                let result = RequestBuilder::new("http://localhost:3000/api/create_notification")
                    .method(Method::POST)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&post_data).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match result {
                    Ok(resp) => {
                        if let Some(editor) = editor_ref_clone.cast::<HtmlDivElement>() {
                            editor.set_inner_html("");
                            content_clone.set("".to_string());
                        }

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
        <div style="display: flex; flex-direction: column; align-items: center; gap: 16px; padding: 20px; font-family: Arial, sans-serif; background-color: #f9f9f9;">
            <div
                ref={editor_ref}
                contenteditable="true"
                style="border: 2px solid #ddd; padding: 12px; min-height: 120px; width: 100%; white-space: pre-wrap;
                        border-radius: 8px; background-color: white; box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.1); outline: none;"
                oninput={oninput}
            />

            <div style="display: flex; justify-content: space-between; width: 100%;">
                <label style="cursor: pointer; background-color: #007bff; color: white; padding: 10px 16px; border-radius: 6px; font-size: 14px;
                              display: inline-block; text-align: center; transition: background 0.3s;">
                    {"Upload Image"}
                    <input type="file" accept="image/*" onchange={onchange_file} style="display: none;" />
                </label>

                <button onclick={on_submit}
                    style="background-color: #28a745; color: white; padding: 10px 16px; border: none; border-radius: 6px;
                           font-size: 16px; cursor: pointer; transition: background 0.3s;">
                    {"Send"}
                </button>
            </div>
        </div>
    }
}
