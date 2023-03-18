use crate::{
    components::{emitters_window::emitters_window::EmittersWindow, sidebar::sidebar::Sidebar},
    utils::console_log,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Callback, Html};

#[function_component(Home)]
pub fn home() -> Html {
    let selected_facility_id = use_state(|| Uuid::nil());

    let facility_id = Rc::new(*selected_facility_id);

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("prop id changed: {:#?}", u);
    //     },
    //     facility_id.clone(),
    // );

    // Create a callback to be passed down as a prop
    let on_facility_click =
        Callback::from(move |facility_id: Uuid| selected_facility_id.set(facility_id));

    html! {
        <div class={classes!("data-window")}>
            <Sidebar {on_facility_click} facility_id={facility_id.clone()} />
            <EmittersWindow facility_id={facility_id.clone()} />
        </div>
    }
}
