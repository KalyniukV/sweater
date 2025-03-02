use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AuthButtonsProps {
    pub on_login: Callback<MouseEvent>,
    pub on_signup: Callback<MouseEvent>,
}

#[function_component(AuthButtons)]
pub fn auth_buttons(props: &AuthButtonsProps) -> Html {
    html! {
        <div>
            <button onclick={props.on_login.clone()}>{"Login"}</button>
            <button onclick={props.on_signup.clone()}>{"Signup"}</button>
        </div>
    }
}