use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::{
    author_list::AuthorList, home::Home, page_not_found::PageNotFound, post_list::PostList,
    users::Users,
};
// use crate::pages::home::Home;
// use crate::pages::page_not_found::PageNotFound;
// use crate::pages::post_list::PostList;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts")]
    Posts,
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[at("/users")]
    Users,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
        }
        Route::Users => {
            html! { <Users /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
