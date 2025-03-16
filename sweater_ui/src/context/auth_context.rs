use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub logged_in: bool,
    pub user_id: String,
    pub username: String,
}

#[derive(Properties, PartialEq)]
pub struct AuthContextProviderProps {
    pub children: Children,
}

#[function_component(AuthContextProvider)]
pub fn auth_context_provider(props: &AuthContextProviderProps) -> Html {
    let auth_context = use_state(|| AuthContext {
        logged_in: false,
        user_id: String::new(),
        username: String::new(),
    });

    let auth_context_clone = auth_context.clone();

    html! {
        <ContextProvider<UseStateHandle<AuthContext>> context={auth_context_clone}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<AuthContext>>>
    }
}