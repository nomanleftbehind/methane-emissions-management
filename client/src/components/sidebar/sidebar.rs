use crate::components::sidebar::facility::FacilityComp;
use crate::hooks::use_query;
use crate::pages::ModalVariant;
use crate::{
    hooks::QueryResponse,
    models::queries::facility::{
        all_facilities::{ResponseData, Variables},
        AllFacilities,
    },
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_facility_click: Callback<Uuid>,
    pub facility_id: Option<Rc<Uuid>>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(Sidebar)]
pub fn sidebar(
    Props {
        on_facility_click,
        facility_id,
        modal_variant_handle,
    }: &Props,
) -> Html {
    let get_facilities = use_query::<AllFacilities>(Variables);

    let view = match get_facilities {
        QueryResponse {
            data: Some(ResponseData { all_facilities }),
            ..
        } => {
            let view = all_facilities.into_iter().map(|facility| {
                let id = facility.id;
                html! {
                    <FacilityComp
                        key={id.to_string()}
                        {facility}
                        {on_facility_click}
                        {facility_id}
                    />
                }
            });
            html! { for view }
        }
        QueryResponse {
            error: Some(error), ..
        } => {
            modal_variant_handle.emit(Some(ModalVariant::Error(error)));
            Html::default()
        }
        _ => Html::default(),
    };

    html! {
        <nav class={classes!("sidebar")} role="navigation">
            <ol class={classes!("sidebar-list")}>
                { view }
            </ol>
        </nav>
    }
}
