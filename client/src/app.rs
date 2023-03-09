use crate::{
    components::{nav::Nav, user::Users},
    pages::{Home, Register},
};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    // #[at("/controllers/:facility_id")]
    // Controllers { facility_id: Uuid },
    #[at("/users")]
    Users,
    #[at("/register")]
    Register,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            // <div>{ facility_display }</div>
            <main class="main">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }
        // Route::Controllers { facility_id } => {
        //     html! { <ControllersPage {facility_id} /> }
        // }
        Route::Users => {
            html! { <Users /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
    }
}
