use yew::prelude::*;
use crate::components::auth::AuthButtons;
use crate::components::auth::LoginForm;
use crate::components::auth::SignupForm;

#[derive(Clone, PartialEq)]
enum FormType {
    None,
    Login,
    Signup,
}

#[function_component(AuthenticationPage)]
pub fn authentication_page() -> Html {
    let form_type = use_state(|| FormType::None);

    let show_login = {
        let form_type = form_type.clone();
        Callback::from(move |_| form_type.set(FormType::Login))
    };

    let show_signup = {
        let form_type = form_type.clone();
        Callback::from(move |_| form_type.set(FormType::Signup))
    };

    let render_form = {
        let form_type = form_type.clone();
        move || match *form_type {
            FormType::Login => html! { <LoginForm /> },
            FormType::Signup => html! { <SignupForm /> },
            FormType::None => html! { <AuthButtons on_login={show_login.clone()} on_signup={show_signup.clone()} /> },
        }
    };

    html! {
        <div>
            {render_form()}
        </div>
    }
}