mod about;
pub mod methane_emission_sources;
pub mod navigation_bar;
pub mod pneumatic_instruments;
mod register;

pub use about::About;
pub use register::Register;

use crate::routes::{
    methane_emission_sources::FacilityList, navigation_bar::NavigationBar,
    pneumatic_instruments::pneumatic_instrument_list::PneumaticInstruments,
};
use leptos::*;
use leptos_router::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExampleContext(i32);

#[component]
pub fn RouterExample() -> impl IntoView {
    log::debug!("rendering <RouterExample/>");

    // contexts are passed down through the route tree
    provide_context(ExampleContext(0));

    view! {
        <Router>
            <NavigationBar/>
            <main class="main">
                // <div class="data-window">
                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                    class="data-window"
                >
                    <Route path="" view=FacilityList>
                        <Route path=":id" view=PneumaticInstruments/>
                        <Route path="/" view=|| view! { <p>"Select a contact."</p> }/>
                    </Route>
                    <Route path="about" view=|| view! { <About/> }/>
                    <Route path="register" view=|| view! { <Register/> }/>
                    <Route path="redirect-home" view=|| view! { <Redirect path="/"/> }/>
                </AnimatedRoutes>
            // </div>
            </main>
        </Router>
    }
}
