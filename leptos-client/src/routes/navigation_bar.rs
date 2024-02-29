use leptos::*;
use leptos_router::*;

#[component]
pub fn navigation_bar() -> impl IntoView {
    provide_context(0);
    // let user_ctx = use_context::<UserContext>().unwrap();

    // Nav is the only common UserContext wrapped component to all other components,
    // so here a query is run every time browser is refreshed to find currently logged in user and update UserContext.
    // use_effect_with_deps(
    //     move |_| {
    //         wasm_bindgen_futures::spawn_local(async move {
    //             if let QueryResponse {
    //                 data: Some(ResponseData { me: Some(me) }),
    //                 ..
    //             } = lazy_query::<Me>(Variables).await
    //             {
    //                 user_ctx.dispatch(Some(me.into()));
    //             }
    //         });
    //     },
    //     (),
    // );

    view! {
        <nav class="navigation-bar" role="navigation">
            <div class="navbar-brand">
                <h1>"Methane Emissions Management App"</h1>
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <div class="navbar-button-wrapper">
                        <A exact=true href="sources" class="navbar-button">
                            "Sources"
                        </A>
                    </div>
                    <div class="navbar-button-wrapper">
                        <A href="about" class="navbar-button">
                            "About"
                        </A>
                    </div>
                    <div class="navbar-button-wrapper">
                        <A href="users" class="navbar-button">
                            "Users"
                        </A>
                    </div>
                    <div class="navbar-button-wrapper">
                        <A href="register" class="navbar-button">
                            "Register"
                        </A>
                    </div>
                    <div class="navbar-button-wrapper">
                        <A href="redirect-home" class="navbar-button">
                            "Redirect to Home"
                        </A>
                    </div>
                </div>
            // <Dropdown />
            </div>
        </nav>
    }
}
