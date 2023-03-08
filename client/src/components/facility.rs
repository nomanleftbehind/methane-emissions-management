use common::FacilityType;
use yew::{classes, function_component, html, Callback, Html, Properties};
use yew_router::prelude::{use_navigator, use_route, Link};

use crate::{utils::console_log, Route};

#[derive(PartialEq, Clone)]
pub struct Facility {
    pub id: uuid::Uuid,
    pub idpa: String,
    pub name: String,
    pub r#type: FacilityType,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub row_num: usize,
    pub facility: Facility,
    pub on_facility_click: Callback<Facility>,
}

#[function_component(FacilityComp)]
pub fn facility_comp(
    Props {
        facility,
        row_num,
        on_facility_click,
    }: &Props,
) -> Html {
    let on_facility_click = on_facility_click.clone();
    let facility = facility.clone();
    let facility_display = facility.clone().name;

    let fi = facility.clone().id;

    let e = use_route::<Route>().unwrap_or(Route::Home);

    let route = match e {
        Route::Controllers { .. } => Route::Controllers { facility_id: fi },
        t => t,
    };

    // let navigator = use_navigator().unwrap();
    // let go_to_first_post_button = {
    //     // let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&route));
    //     html! {
    //         <button {onclick}>{"click to go the first post"}</button>
    //     }
    // };


    let onclick = Callback::from(move |_| {
        let facility = facility.clone();
        on_facility_click.emit(facility);
        // navigator.push(&route);
    });

    let style = format!("grid-column: 1; grid-row: {};", row_num + 1);
    html! {
        <button {style} {onclick}>
            <Link<Route> classes={classes!("navbar-item")} to={route}>
                { facility_display }
            </Link<Route>>
        </button>
    }
}
