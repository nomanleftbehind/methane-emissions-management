use leptos::*;
use leptos_router::*;

use crate::{routes::methane_emission_sources::FacilityList, Contact};

// You can define other routes in their own component.
// Use a #[component(transparent)] that returns a <Route/>.
#[component(transparent)]
pub fn FacilityRoutes() -> impl IntoView {
    view! {
        <Route path="" view=|| view! { <FacilityList/> }>
            <Route path=":id" view=|| view! { <Contact/> }/>
            <Route path="/" view=|| view! { <p>"Select a contact."</p> }/>
        </Route>
    }
}
