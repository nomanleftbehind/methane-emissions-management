use crate::{
    components::{
        contexts::user_context::UserProvider, navigation::navigation_bar::NavigationBar,
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
        <>
            <BrowserRouter>
                <UserProvider>
                    <NavigationBar />
                    <main class="main">
                        <Switch<Route> render={switch} />
                    </main>
                </UserProvider>
            </BrowserRouter>
            <dialog id="modal-root"></dialog>
        </>
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
    #[not_found]
    #[at("/404")]
    NotFound,
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
        Route::NotFound => html! { <h1>{ "404" }</h1> },
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
        <>
            <Router history={history}>
                <UserProvider>
                    <NavigationBar />
                    <main class="main">
                        <Switch<Route> render={switch} />
                    </main>
                </UserProvider>
            </Router>
            <dialog id="modal-root"></dialog>
        </>
    }
}
