use crate::{
    components::contexts::user_context::UserContext,
    hooks::{lazy_query, QueryResponse},
    models::mutations::user::{
        logout::{ResponseData, Variables},
        Logout as LogoutUser,
    },
};
use yew::{classes, function_component, html, use_context, Callback, Html};

#[function_component(Logout)]
pub fn logout() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();

    let onclick = Callback::from(move |_| {
        let user_ctx = user_ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let QueryResponse {
                data: Some(ResponseData { logout: _ }),
                ..
            } = lazy_query::<LogoutUser>(Variables).await
            {
                user_ctx.dispatch(None);
            };
        });
    });

    html! {
        <div class={classes!("navbar-button-wrapper")}>
            <button class={classes!("navbar-button")} {onclick}>{ "Logout" }</button>
        </div>
    }
}
