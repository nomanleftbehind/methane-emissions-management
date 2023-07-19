use crate::{
    components::{contexts::user_context::UserContext, navigation::dropdown::Dropdown},
    hooks::{lazy_query, QueryResponse},
    models::queries::user::{
        me::{ResponseData, Variables},
        Me,
    },
    Route,
};
use yew::{classes, function_component, html, use_context, use_effect_with_deps, Html};
use yew_router::components::Link;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();

    // Nav is the only common UserContext wrapped component to all other components,
    // so here a query is run every time browser is refreshed to find currently logged in user and update UserContext.
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let QueryResponse {
                    data: Some(ResponseData { me: Some(me) }),
                    ..
                } = lazy_query::<Me>(Variables).await
                {
                    user_ctx.dispatch(Some(me.into()));
                }
            });
        },
        (),
    );

    html! {
        <nav class={classes!("navigation-bar")} role="navigation">
            <div class={classes!("navbar-brand")}>
                <h1>{ "Methane Emissions Management App" }</h1>
            </div>
            <div class={classes!("navbar-menu"/*, active_class*/)}>
                <div class="navbar-start">
                    <div class={classes!("navbar-button-wrapper")}>
                        <Link<Route> classes={classes!("navbar-button")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                    </div>
                    <div class={classes!("navbar-button-wrapper")}>
                        <Link<Route> classes={classes!("navbar-button")} to={Route::Users}>
                            { "Users" }
                        </Link<Route>>
                    </div>
                    <div class={classes!("navbar-button-wrapper")}>
                        <Link<Route> classes={classes!("navbar-button")} to={Route::Register}>
                            { "Register" }
                        </Link<Route>>
                    </div>
                </div>
                <Dropdown />
            </div>
        </nav>
    }
}
