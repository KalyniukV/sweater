use crate::routes::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

use components::pages::authentication_page::AuthenticationPage;
use crate::context::auth_context::AuthContextProvider;

mod components;
mod models;
mod routes;
mod context;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AuthContextProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </AuthContextProvider>
    }
}

