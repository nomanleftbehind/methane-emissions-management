use crate::{
    hooks::{use_query, QueryResponse},
    models::queries::controller::{
        get_controllers::{ControllersBy, ResponseData, Variables},
        GetControllers,
    },
    utils::console_log,
};
use uuid::Uuid;
use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub facility_id: Uuid,
}

#[function_component(ControllersPage)]
pub fn controllers_page(Props { facility_id }: &Props) -> Html {

    console_log!("Ctr fac: {}", &facility_id);
    let var = Variables {
        by: ControllersBy {
            facility_id: *facility_id,
        },
    };

    let get_controllers = use_query::<GetControllers>(var);

    let r = match get_controllers {
        QueryResponse {
            data: Some(ResponseData { controllers_by }),
            ..
        } => {
            let controllers_iter = controllers_by.into_iter().map(|c| {
                let m = c.manufacturer.map_or("".to_string(), |v| v.manufacturer);
                html! {
                    <>
                        <div>{ c.id }</div>
                        <div>{ c.model }</div>
                        <div>{ c.serial_number }</div>
                        <div>{ m }</div>
                    </>
                }
            });

            html! { for controllers_iter }
        }
        QueryResponse { error: Some(e), .. } => {
            html! {e}
        }
        _ => {
            html! {}
        }
    };

    html! {r}
}
