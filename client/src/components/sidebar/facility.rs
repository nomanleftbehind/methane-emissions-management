use crate::models::queries::facility::all_facilities::AllFacilitiesAllFacilities;
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility: AllFacilitiesAllFacilities,
    pub on_facility_click: Callback<Uuid>,
    pub facility_id: Option<Rc<Uuid>>,
}

#[function_component(FacilityComp)]
pub fn facility_comp(
    Props {
        facility,
        on_facility_click,
        facility_id,
    }: &Props,
) -> Html {
    let on_facility_click = on_facility_click.clone();

    let id = facility.id;

    let onclick = Callback::from(move |_| {
        on_facility_click.emit(id);
    });

    html! {
        <li class={classes!("sidebar-button-container")}>
            <button class={classes!("sidebar-button", facility_id.as_ref().and_then(|u| (u.as_ref() == &id).then(|| "active")))} {onclick}>
                { &facility.name }
            </button>
        </li>
    }
}
