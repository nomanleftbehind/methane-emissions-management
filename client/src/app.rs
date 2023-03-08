use crate::{
    components::{facility::FacilityNav, nav::Nav, user::Users},
    pages::{ControllersPage, Home, Register},
};
// use uuid::Uuid;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    // #[at("/facilities/:id/controllers")]
    #[at("/controllers")]
    Controllers/*{ id: Uuid }*/,
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
        Route::Controllers/*{ id }*/ => {
            html! { <ControllersPage/*{id}*/ /> }
        }
        Route::Users => {
            html! { <Users /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
    }
}
