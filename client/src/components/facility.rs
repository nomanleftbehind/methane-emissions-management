use common::FacilityType;
use uuid::Uuid;
use yew::{function_component, html, Callback, Html, Properties};

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

    let id = (*facility).id;

    let onclick = Callback::from(move |_| {
        on_facility_click.emit(id);
    });

    let style = format!("grid-column: 1; grid-row: {};", row_num + 1);
    html! {
        <button {style} {onclick}>
                { &facility.name }
        </button>
    }
}
