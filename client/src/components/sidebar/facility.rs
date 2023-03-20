use common::FacilityType;
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Clone)]
pub struct Facility {
    pub id: uuid::Uuid,
    pub idpa: String,
    pub name: String,
    pub r#type: FacilityType,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility: Facility,
    pub on_facility_click: Callback<Uuid>,
    pub facility_id: Rc<Uuid>,
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
            <button class={classes!("sidebar-button", (facility_id.as_ref() == &id).then(|| "green"))} {onclick}>
                { &facility.name }
            </button>
        </li>
    }
}
