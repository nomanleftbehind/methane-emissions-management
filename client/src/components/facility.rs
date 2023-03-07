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
    id: String,
    idpa: String,
    name: String,
    r#type: FacilityType,
}

#[derive(Properties, PartialEq)]
pub struct FacilityProps {
    facility: Facility,
}

#[function_component(FacilityComp)]
pub fn facility_comp(
    FacilityProps {
        facility:
            Facility {
                id,
                idpa: _,
                name,
                r#type,
            },
    }: &FacilityProps,
) -> Html {
    html! {
        <>
            <div>{ id }</div>
            <div>{ name }</div>
            <div>{ r#type }</div>
        </>
    }
}

#[function_component(FacilityNav)]
pub fn facility_nav() -> Html {
    let get_facilities = use_query::<AllFacilities>(Variables);

    match get_facilities {
        Ok(QueryResponse {
            data: Some(ResponseData { all_facilities }),
            ..
        }) => {
            let r = all_facilities.into_iter().map(
                |AllFacilitiesAllFacilities {
                     id,
                     idpa,
                     name,
                     type_,
                 }| {
                    let facility = Facility {
                        id: id.clone(),
                        idpa,
                        name,
                        r#type: type_,
                    };
                    html! {
                        <FacilityComp key={id} {facility} />
                    }
                },
            );
            html! { for r }
        }
        Ok(_) => html! {},
        Err(e) => {
            html! {e}
        }
    }
}
