mod about;
pub mod methane_emission_sources;
pub mod navigation_bar;
pub mod pneumatic_instruments;
mod register;

pub use about::About;
pub use register::Register;

use crate::routes::{
    methane_emission_sources::{FacilityList, SourcesNavigationBar},
    navigation_bar::NavigationBar,
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

                    <Route path="/sources" view=FacilityList>
                        <Route path=":id" view=SourcesNavigationBar>
                            <Route path="" view=|| view! { <p>"Select a source."</p> }/>
                            <Route path="pneumatic_instruments" view=PneumaticInstruments/>
                        </Route>
                        <Route
                            path=""
                            view=|| view! { <p>"Select a contact to view more info."</p> }
                        />
                    </Route>
                    // <Route path="/sources" view=FacilityList>
                    // <Route path=":id" view=SourcesNavigationBar>
                    // <Route path="pneumatic_instruments" view=PneumaticInstruments/>
                    // <Route path="" view=|| view! { <p>"Select a source."</p> }/>
                    // </Route>
                    // <Route path="" view=|| view! { <p>"Select ID"</p> }/>
                    // </Route>
                    <Route path="about" view=|| view! { <About/> }/>
                    <Route path="/" view=|| view! { <p>"Home"</p> }/>
                    <Route path="register" view=|| view! { <Register/> }/>
                    <Route path="redirect-home" view=|| view! { <Redirect path="/"/> }/>
                </AnimatedRoutes>
            // </div>
            </main>
        </Router>
    }
}
