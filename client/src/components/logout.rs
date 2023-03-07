use yew::{function_component, html, Callback, Html};
use yew_hooks::use_async;

use crate::{
    hooks::{use_query_async, QueryResponse},
    models::mutations::user::{
        logout::{ResponseData, Variables},
        Logout as LogoutUser,
    },
    utils::console_log::console_log,
};

#[function_component(Logout)]
pub fn logout() -> Html {
    let state = use_async(async move { use_query_async::<LogoutUser>(Variables).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    let a = &state.data;

    if let Some(QueryResponse {
        data: Some(ResponseData { logout }),
        ..
    }) = a
    {
        console_log!("logout {}", logout);
    };

    html! {
        <button class={"logout-button"} {onclick}>{ "Logout" }</button>
    }
}
