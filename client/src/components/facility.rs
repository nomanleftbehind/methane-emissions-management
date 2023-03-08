use crate::hooks::use_query;
use crate::{
    hooks::QueryResponse,
    models::queries::facility::{
        all_facilities::{AllFacilitiesAllFacilities, ResponseData, Variables},
        AllFacilities,
    },
};
use common::FacilityType;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Clone)]
pub struct Facility {
    id: uuid::Uuid,
    idpa: String,
    name: String,
    r#type: FacilityType,
}

#[derive(Properties, PartialEq)]
pub struct FacilityProps {
    row_num: usize,
    facility: Facility,
}

#[function_component(FacilityComp)]
pub fn facility_comp(
    FacilityProps {
        facility:
            Facility {
                id: _,
                idpa: _,
                name,
                r#type: _,
            },
        row_num,
    }: &FacilityProps,
) -> Html {
    let style = format!("grid-column: 1; grid-row: {};", row_num + 1);
    html! {
        <button {style}>{ name }</button>
    }
}

#[function_component(FacilityNav)]
pub fn facility_nav() -> Html {
    let get_facilities = use_query::<AllFacilities>(Variables);

    let inner = match get_facilities {
        Ok(QueryResponse {
            data: Some(ResponseData { all_facilities }),
            ..
        }) => {
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
                        id: id.clone(),
                        idpa,
                        name,
                        r#type: type_,
                    };
                    html! {
                        <FacilityComp
                            key={id.to_string()}
                            {facility}
                            {row_num}
                        />
                    }
                },
            );
            html! { for r }
        }
        Ok(_) => html! {},
        Err(e) => {
            html! {e}
        }
    };

    html! {
        <nav class="facility-nav">{ inner }</nav>
    }
}
