use crate::{components::logout::Logout, Route};
use yew::{classes, function_component, html, Html};
use yew_router::components::Link;

#[function_component(Nav)]
pub fn nav() -> Html {
    // console_log!("Nav fac: {}", &facility_id);

    // let navbar_active = use_state_eq(|| false);

    // let toggle_navbar = {
    //     let navbar_active = navbar_active.clone();

    //     Callback::from(move |_| {
    //         navbar_active.set(!*navbar_active);
    //     })
    // };

    // let active_class = if !*navbar_active { "is-active" } else { "" };

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
                    // <Link<Route> classes={classes!("navbar-item")} to={Route::Controllers {facility_id}}>
                    //     { "Controllers" }
                    // </Link<Route>>
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
