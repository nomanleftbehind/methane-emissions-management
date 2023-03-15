use crate::{
    components::{
        contexts::user_context::{Producer, UserProvider},
        nav::Nav,
        user::Users,
    },
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
            <UserProvider>
                <Nav />
                <Producer />
                <main class="main">
                    <Switch<Route> render={switch} />
                </main>
            </UserProvider>
        </BrowserRouter>
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
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

    // println!("props: {:#?}", &props);

    history.push(&*props.url);

    html! {
        <Router history={history}>
            <UserProvider>
                <Nav />
                <Producer />
                <main class="main">
                    <Switch<Route> render={switch} />
                </main>
            </UserProvider>
        </Router>
    }
}
