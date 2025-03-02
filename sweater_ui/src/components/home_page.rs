use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{CreateNotification, GetNotifications};
use crate::context::auth_context::AuthContext;
use crate::routes::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let auth_context = use_context::<UseStateHandle<AuthContext>>().unwrap();
    let navigator = use_navigator().unwrap();

    if !auth_context.logged_in {
        navigator.push(&Route::Authentication);
        return html! {};
    }

    let logout = {
        let auth_context = auth_context.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let mut new_auth_context = (*auth_context).clone();
            new_auth_context.logged_in = false;
            new_auth_context.username = String::new();
            auth_context.set(new_auth_context);
            navigator.push(&Route::Authentication);
        })
    };

    let create_notification_url = "http://localhost:3000/create_notification";
    let get_notifications_url = "http://localhost:3000/notifications";

    html! {
        <div>
            <h1>{"Welcome, to sweater "}{auth_context.username.clone()}</h1>
            <button onclick={logout}>{"Logout"}</button>

            <CreateNotification url={create_notification_url} />
            <GetNotifications url={get_notifications_url}/>
        </div>
    }
}