use crate::{
    components::{nav::Nav, user::Users, facility::FacilityNav},
    pages::{Home, Register},
};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
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
            <main>
                <Switch<Route> render={switch} />
            </main>
            <FacilityNav />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Users => {
            html! { <Users /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
    }
}
