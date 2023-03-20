use crate::{
    components::contexts::user_context::UserContext,
    hooks::{lazy_query, QueryResponse},
    models::mutations::user::{
        logout::{ResponseData, Variables},
        Logout as LogoutUser,
    },
};
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_logout: Callback<()>,
}

#[function_component(Logout)]
pub fn logout(Props { on_logout }: &Props) -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();

    let on_logout = on_logout.clone();

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
        on_logout.emit(());
    });

    html! {
        <button role="menuitem" {onclick}>{ "Logout" }</button>
    }
}
