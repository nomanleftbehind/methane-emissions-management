use components::{login::Login, user::Users};
use yew::prelude::*;

pub mod components;
pub mod hooks;
pub mod models;
pub mod utils;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Emissions App" }</h1>
            <div>
                <h3>{"Users"}</h3>
                <Login />
                <Users />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
