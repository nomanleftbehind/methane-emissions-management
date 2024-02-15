mod about;
pub mod methane_emission_sources;
mod register;

pub use about::About;
pub use register::Register;

use crate::routes::methane_emission_sources::FacilityRoutes;
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
            <nav>
                // ordinary <a> elements can be used for client-side navigation
                // using <A> has two effects:
                // 1) ensuring that relative routing works properly for nested routes
                // 2) setting the `aria-current` attribute on the current link,
                // for a11y and styling purposes
                <A exact=true href="/">
                    "Home"
                </A>
                <A href="about">"About"</A>
                <A href="register">"Register"</A>
                <A href="redirect-home">"Redirect to Home"</A>
            </nav>
            <main class="main">
                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                >
                    <FacilityRoutes/>
                    <Route path="about" view=|| view! { <About/> }/>
                    <Route path="register" view=|| view! { <Register/> }/>
                    <Route path="redirect-home" view=|| view! { <Redirect path="/"/> }/>
                </AnimatedRoutes>
            </main>
        </Router>
    }
}
