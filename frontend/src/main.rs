#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod pages;
mod util;

use console_error_panic_hook::set_once as set_panic_hook;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

use pages::{controllers::Controllers, home::Home, users::Users};

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Route {
    #[to = "/users"]
    Users,
    #[to = "/controllers"]
    Controllers,
    #[to = "/"]
    Home,
}

fn switch(switch: Route) -> Html {
    match switch {
        Route::Users => {
            html! { <Users/> }
        }
        Route::Controllers => {
            html! { <Controllers/> }
        }
        Route::Home => {
            html! { <Home /> }
        }
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        let home_cls = "nav";

        html! {
            <>
                <div class="logo-title">
                    <img src="imgs/budshome.png" />
                    <strong>{ "frontend" }</strong>
                </div>
                <div class=home_cls>
                    <Anchor route=Route::Users>
                        { "Users" }
                    </Anchor>
                    <span class="placeholder">{ " - " }</span>
                    <Anchor route=Route::Controllers>
                        { "Controllers" }
                    </Anchor>
                    <span class="placeholder">{ " - " }</span>
                    <Anchor route=Route::Home>
                        { "Home" }
                    </Anchor>
                </div>
                <main>
                    <Router<Route> render=Router::render(switch) />
                </main>
            </>
        }
    }
}

fn main() {
    set_panic_hook();

    yew::start_app::<App>();
}
