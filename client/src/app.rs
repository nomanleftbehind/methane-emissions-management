use crate::{
    components::{nav::Nav, user::Users},
    pages::{Home, Register},
};
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            // <div>{ "Hi" }</div>
            <main class="main">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

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

pub fn switch(route: Route) -> Html {
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

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);

    html! {
        <Router history={history}>
            <Nav />

            <main class="main">
                <Switch<Route> render={switch} />
            </main>
        </Router>
    }
}
