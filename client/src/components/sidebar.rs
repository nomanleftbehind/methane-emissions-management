use crate::components::facility::{Facility, FacilityComp};
use crate::hooks::use_query;
use crate::{
    hooks::QueryResponse,
    models::queries::facility::{
        all_facilities::{AllFacilitiesAllFacilities, ResponseData, Variables},
        AllFacilities,
    },
};
use uuid::Uuid;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_facility_click: Callback<Uuid>,
}

#[function_component(Sidebar)]
pub fn sidebar(Props { on_facility_click }: &Props) -> Html {
    let get_facilities = use_query::<AllFacilities>(Variables);

    let inner = match get_facilities {
        QueryResponse {
            data: Some(ResponseData { all_facilities }),
            ..
        } => {
            let r = all_facilities.into_iter().enumerate().map(
                |(
                    row_num,
                    AllFacilitiesAllFacilities {
                        id,
                        idpa,
                        name,
                        type_,
                    },
                )| {
                    let facility = Facility {
                        id,
                        idpa,
                        name,
                        r#type: type_,
                    };
                    html! {
                        <FacilityComp
                            key={id.to_string()}
                            {facility}
                            {row_num}
                            {on_facility_click}
                        />
                    }
                },
            );
            html! { for r }
        }
        QueryResponse { error: Some(e), .. } => html! {e},
        _ => {
            html! {}
        }
    };

    html! {
        <nav class="sidebar">{ inner }</nav>
    }
}
