use yew::{html, Html};
use yew_router::Routable;
use crate::components::auth_error_page::AuthErrorPage;
use crate::components::home_page::HomePage;
use crate::AuthenticationPage;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Authentication,
    #[at("/home")]
    Home,
    #[at("/auth_error")]
    AuthError,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Authentication => html! { <AuthenticationPage /> },
        Route::Home => html! { <HomePage /> },
        Route::AuthError => html! {<AuthErrorPage />}
    }
}