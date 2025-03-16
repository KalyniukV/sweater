use yew::{html, Html};
use yew_router::Routable;
use crate::components::pages::home_page::HomePage;
use crate::AuthenticationPage;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Authentication,
    #[at("/home")]
    Home
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Authentication => html! { <AuthenticationPage /> },
        Route::Home => html! { <HomePage /> }
    }
}