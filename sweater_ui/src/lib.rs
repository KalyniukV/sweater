use yew::prelude::*;
use crate::components::{CreateNotification, GetNotifications};

mod components;


#[function_component(App)]
pub fn app() -> Html {
    let create_notification_url = "http://localhost:3000/create_notification";
    let get_notifications_url = "http://localhost:3000/notifications";

    html! {
        <>
            <CreateNotification url={create_notification_url} />
            <GetNotifications url={get_notifications_url}/>
        </>
    }
}