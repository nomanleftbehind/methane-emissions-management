use yew::{function_component, html, use_context, Callback, Html};
use yew_hooks::use_async;

use crate::{
    components::user_ctx::UserContext,
    hooks::{use_query_async, QueryResponse},
    models::mutations::user::{
        logout::{ResponseData, Variables},
        Logout as LogoutUser,
    }, utils::console_log,
};

#[function_component(Logout)]
pub fn logout() -> Html {
    let state = use_async(async move { use_query_async::<LogoutUser>(Variables).await });
    let user_ctx = use_context::<UserContext>().unwrap();

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
            user_ctx.dispatch(None);
            console_log!("Logout");
        })
    };

    let a = &state.data;

    if let Some(QueryResponse {
        data: Some(ResponseData { logout: _ }),
        ..
    }) = a
    {};

    html! {
        <button class={"logout-button"} {onclick}>{ "Logout" }</button>
    }
}
