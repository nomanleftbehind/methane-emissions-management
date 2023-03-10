use std::collections::HashMap;
use yew_router::{history::{AnyHistory, History, MemoryHistory}, Router};
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



#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Nav />

            <main class="main">
                <Switch<Route> render={switch} />
            </main>
        </Router>
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
