use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AuthButtonsProps {
    pub on_login: Callback<MouseEvent>,
    pub on_signup: Callback<MouseEvent>,
}

#[function_component(AuthButtons)]
pub fn auth_buttons(props: &AuthButtonsProps) -> Html {
    html! {
        <div style="display: flex; gap: 12px; justify-content: center; align-items: center; height: 100vh; background-color: #f9f9f9;">
            <button onclick={props.on_login.clone()}
                style="background-color: #007bff; color: white; padding: 10px 16px; border: none; border-radius: 6px;
                       font-size: 16px; cursor: pointer; transition: background 0.3s;">
                {"Login"}
            </button>

            <button onclick={props.on_signup.clone()}
                style="background-color: #28a745; color: white; padding: 10px 16px; border: none; border-radius: 6px;
                       font-size: 16px; cursor: pointer; transition: background 0.3s;">
                {"Signup"}
            </button>
        </div>
    }
}