use crate::{
    components::{contexts::user_context::UserContext, logout::Logout},
    hooks::{lazy_query, QueryResponse},
    models::queries::user::{
        me::{ResponseData, Variables},
        Me,
    },
    Route,
};
use yew::{classes, function_component, html, use_context, Html};
use yew_hooks::use_effect_once;
use yew_router::components::Link;

#[function_component(Nav)]
pub fn nav() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();

    // Nav is the only common UserContext wrapped component to all other components,
    // so here a query is run every time browser is refreshec to find currently logged in user and update UserContext.
    use_effect_once(move || {
        wasm_bindgen_futures::spawn_local(async move {
            if let QueryResponse {
                data: Some(ResponseData { me: Some(me) }),
                ..
            } = lazy_query::<Me>(Variables).await
            {
                user_ctx.dispatch(Some(me.into()));
            }
        });
        || ()
    });

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-4">{ "Emissions App" }</h1>

                // <button class={classes!("navbar-burger", "burger", active_class)}
                //     aria-label="menu" aria-expanded="false"
                //     onclick={toggle_navbar}
                // >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                // </button>
            </div>
            <div class={classes!("navbar-menu"/*, active_class*/)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Users}>
                        { "Users" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Register}>
                        { "Register" }
                    </Link<Route>>
                    <Logout />
                </div>
            </div>
        </nav>
    }
}
