use crate::context::auth_context::AuthContext;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::create_notification::CreateNotification;
use crate::components::pages::get_notifications::GetNotifications;

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

    html! {
        <div style="position: relative; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 10px; background-color: #f9f9f9; font-family: Arial, sans-serif;">

            <button onclick={logout}
                style="position: absolute; top: 20px; right: 20px; background-color: #dc3545; color: white; padding: 8px 16px;
                           border: none; border-radius: 6px; font-size: 14px; cursor: pointer; transition: background 0.3s;">
              {"Logout"}
            </button>

            <div style="width: 80%;">
                <h2 style="color: #333; font-size: 1.5rem;">
                        { &auth_context.username }
                </h2>
            </div>

            <div style="width: 80%;padding: 20px; background-color: white; border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); margin-bottom: 20px;">
                <CreateNotification user_id={auth_context.user_id.clone()} />
            </div>

            <div style="width: 80%;padding: 20px; background-color: white; border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);">
                <GetNotifications />
            </div>
        </div>
    }
}