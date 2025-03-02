use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(AuthErrorPage)]
pub fn auth_error_page() -> Html {
    html! {
        <div>
            <h1>{"Authentication Failed"}</h1>
            <p>{"Please check your credentials and try again."}</p>
            <Link<Route> to={Route::Authentication}>{"Go to Login/Signup"}</Link<Route>>
        </div>
    }
}