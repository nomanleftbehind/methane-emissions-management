use crate::{
    components::{facility::Facility, nav::Nav, sidebar::Sidebar, user::Users},
    pages::{ControllersPage, Home, Register}, utils::console_log,
};
use uuid::Uuid;
// use uuid::Uuid;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch, prelude::{use_route, use_navigator}};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/controllers/:facility_id")]
    Controllers { facility_id: Uuid },
    #[at("/users")]
    Users,
    #[at("/register")]
    Register,
}

#[function_component(App)]
pub fn app() -> Html {
    let selected_facility = use_state(|| None);

    let (facility_id, facility_display) = (*selected_facility).clone().map_or((Uuid::nil(), "".to_string()), |f: Facility| (f.id, f.name));






    //  = sf.map_or((Uuid::nil(), "".to_string()), |f: Facility| (f.id, f.name));

    // Create a callback to be passed down as a prop
    let on_facility_click = {
        let selected_facility = selected_facility.clone();
        Callback::from(move |facility: Facility| selected_facility.set(Some(facility)))
    };

    html! {
        <BrowserRouter>
            <Nav {facility_id} />
            <main>
                <Switch<Route> render={switch} />
            </main>
            <div>{ facility_display }</div>
            <Sidebar {on_facility_click} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Controllers { facility_id } => {
            html! { <ControllersPage {facility_id} /> }
        }
        Route::Users => {
            html! { <Users /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
    }
}
