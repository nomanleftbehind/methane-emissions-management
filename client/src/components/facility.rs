use common::FacilityType;
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::utils::gen_style::gen_grid_style;

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
    pub on_facility_click: Callback<Uuid>,
    pub facility_id: Rc<Uuid>,
}

#[function_component(FacilityComp)]
pub fn facility_comp(
    Props {
        facility,
        row_num,
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
        <button class={classes!((facility_id.as_ref() == &id).then(|| "active"))} style={gen_grid_style(1, row_num + 1)} {onclick}>
            { &facility.name }
        </button>
    }
}
